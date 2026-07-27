use std::env;
use std::path::PathBuf;
use std::fmt;

/// Build configuration cho CONA compiler
#[derive(Debug, Clone)]
pub struct ConaBuildConfig {
    pub target_dir: PathBuf,
    pub build_target: String,
    pub is_release: bool,
    pub output_dir: PathBuf,
    pub target_platform: String,
}

/// Build errors
#[derive(Debug)]
pub enum ConaBuildError {
    TargetDirectoryNotFound(PathBuf),
    OutputDirectoryNotFound(PathBuf),
    ExecutionFailed(String),
    IoError(std::io::Error),
}

impl fmt::Display for ConaBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TargetDirectoryNotFound(p) => write!(f, "Target directory not found: {:?}", p),
            Self::OutputDirectoryNotFound(p) => write!(f, "Output directory not found: {:?}", p),
            Self::ExecutionFailed(msg) => write!(f, "Build execution failed: {}", msg),
            Self::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ConaBuildError {}

impl From<std::io::Error> for ConaBuildError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl ConaBuildConfig {
    /// Resolution cascade:
    /// 1. Explicit config (parameter)
    /// 2. Environment variable (HACE_UNI_TARGET_PATH hoặc HACE_UNI_OUTPUT_PATH)
    /// 3. Workspace fallback (CARGO_MANIFEST_DIR)
    pub fn resolve(
        explicit_target: Option<PathBuf>,
        explicit_output: Option<PathBuf>,
    ) -> Result<Self, ConaBuildError> {
        // Resolve target directory
        let target_dir = if let Some(path) = explicit_target {
            // Tier 1: Explicit config
            path
        } else if let Ok(env_path) = env::var("HACE_UNI_TARGET_PATH") {
            // Tier 2: Environment variable
            PathBuf::from(env_path)
        } else {
            // Tier 3: Workspace fallback
            let manifest_dir = env::var("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."));
            
            // Try relative path: engine/hace/uni/target hoặc uni/target
            let candidate = manifest_dir
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.join("engine/hace/uni/target"))
                .or_else(|| manifest_dir.parent().map(|p| p.join("uni/target")))
                .unwrap_or_else(|| manifest_dir.join("target"));
            
            candidate
        };

        // Resolve output directory
        let output_dir = if let Some(path) = explicit_output {
            path
        } else if let Ok(env_path) = env::var("HACE_UNI_OUTPUT_PATH") {
            PathBuf::from(env_path)
        } else {
            target_dir.join("pkg")
        };

        // Canonicalize paths
        let normalized_target = target_dir
            .canonicalize()
            .map_err(|_| ConaBuildError::TargetDirectoryNotFound(target_dir.clone()))?;

        let normalized_output = output_dir
            .canonicalize()
            .map_err(|_| ConaBuildError::OutputDirectoryNotFound(output_dir.clone()))?;

        Ok(Self {
            target_dir: normalized_target,
            build_target: env::var("HACE_WASM_TARGET")
                .unwrap_or_else(|_| "wasm32-unknown-unknown".to_string()),
            is_release: env::var("HACE_BUILD_RELEASE")
                .map(|v| v != "false" && v != "0")
                .unwrap_or(true),
            output_dir: normalized_output,
            target_platform: env::var("HACE_TARGET_PLATFORM")
                .unwrap_or_else(|_| "auto".to_string()),
        })
    }

    /// Legacy support - resolve từ package root
    pub fn from_package_root(package_root: PathBuf) -> Result<Self, ConaBuildError> {
        let target_dir = package_root.join("target");
        let output_dir = package_root.join("pkg");

        Ok(Self {
            target_dir: target_dir.canonicalize().map_err(|_| {
                ConaBuildError::TargetDirectoryNotFound(target_dir.clone())
            })?,
            build_target: "wasm32-unknown-unknown".to_string(),
            is_release: true,
            output_dir,
            target_platform: "auto".to_string(),
        })
    }
}

impl ConaBuildConfig {
    /// Load từ environment hoặc default
    pub fn load() -> Result<Self, ConaBuildError> {
        Self::resolve(None, None)
    }
}

/// WASM builder
pub struct ConaWasmBuilder {
    config: ConaBuildConfig,
    platform: Option<hace_uni_resolver::PlatformKind>,
}

impl ConaWasmBuilder {
    pub fn new(config: &ConaBuildConfig) -> Self {
        Self {
            config: config.clone(),
            platform: None,
        }
    }

    pub fn new_with_platform(config: &ConaBuildConfig, platform: hace_uni_resolver::PlatformKind) -> Self {
        Self {
            config: config.clone(),
            platform: Some(platform),
        }
    }

    pub fn build(&self) -> Result<std::path::PathBuf, ConaBuildError> {
        // Determine target triple based on platform
        let target_triple = match self.platform {
            Some(hace_uni_resolver::PlatformKind::Windows) => "wasm32-unknown-unknown",
            Some(hace_uni_resolver::PlatformKind::Linux) => "wasm32-unknown-unknown",
            Some(hace_uni_resolver::PlatformKind::MacOS) => "wasm32-unknown-unknown",
            Some(hace_uni_resolver::PlatformKind::Web) => "wasm32-unknown-unknown",
            Some(hace_uni_resolver::PlatformKind::Android) => "wasm32-unknown-unknown",
            Some(hace_uni_resolver::PlatformKind::IOS) => "wasm32-unknown-unknown",
            None => &self.config.build_target,
            _ => "wasm32-unknown-unknown",
        };

        let mut args = vec!["build", "--target", target_triple];
        if self.config.is_release {
            args.push("--release");
        }

        let status = std::process::Command::new("cargo")
            .args(&args)
            .current_dir(&self.config.target_dir)
            .status()
            .map_err(|e| ConaBuildError::ExecutionFailed(e.to_string()))?;

        if !status.success() {
            return Err(ConaBuildError::ExecutionFailed(
                "cargo build failed".to_string()
            ));
        }

        // Return path to WASM
        Ok(self.config.target_dir
            .join("wasm32-unknown-unknown")
            .join("release")
            .join("hace_uni_target.wasm"))
    }
}