# hace-uni-config

**SMF ID**: `SMF://hace.uni.config.v1`  
**Intent**: DECLARE_SCHEMA  
**Status**: ACTIVE  
**Layer**: L0 — Build / Produce Plane  
**Role**: CONA Build Configuration  
**WASM Ready**: ❌ (native only)

CONA build configuration resolution cascade.

---

## Overview

`hace-uni-config` provides a three-tier resolution cascade for build configuration paths. It resolves target directories, output directories, and WASM build targets using explicit parameters, environment variables, or workspace fallbacks.

### Resolution Cascade

| Tier | Priority | Source |
|---|---|---|
| 1 | Highest | Explicit parameter |
| 2 | Medium | Environment variable (`HACE_UNI_TARGET_PATH`, `HACE_UNI_OUTPUT_PATH`) |
| 3 | Lowest | Workspace fallback (`CARGO_MANIFEST_DIR`) |

### Key Types

| Type | Description |
|---|---|
| `ConaBuildConfig` | Resolved build configuration: target_dir, build_target, is_release, output_dir, target_platform |
| `ConaWasmBuilder` | WASM builder with platform-aware target triple selection |
| `ConaBuildError` | Error enum: TargetDirectoryNotFound, OutputDirectoryNotFound, ExecutionFailed, IoError |

### Environment Variables

| Variable | Default | Description |
|---|---|---|
| `HACE_UNI_TARGET_PATH` | — | Override target directory |
| `HACE_UNI_OUTPUT_PATH` | — | Override output directory |
| `HACE_WASM_TARGET` | `wasm32-unknown-unknown` | WASM target triple |
| `HACE_BUILD_RELEASE` | `true` | Enable release mode |
| `HACE_TARGET_PLATFORM` | `auto` | Target platform override |

---

## Usage

### Load Configuration

```rust
use haha_uni_config::ConaBuildConfig;

// Tier 1: Explicit parameters
let config = ConaBuildConfig::resolve(
    Some("/path/to/target".into()),
    Some("/path/to/output".into()),
)?;

// Tier 2+3: Environment + workspace fallback
let config = ConaBuildConfig::load()?;
```

### Build WASM

```rust
use haha_uni_config::{ConaBuildConfig, ConaWasmBuilder};
use haha_uni_resolver::PlatformKind;

let config = ConaBuildConfig::load()?;
let builder = ConaWasmBuilder::new_with_platform(&config, PlatformKind::Web);
let wasm_path = builder.build()?;
```

---

## Build

```bash
cd engine/hace/uni/config
cargo build --release
```

---

## Dependencies

- `hace-uni-resolver` (path dependency)
- `serde` 1.0

---

## Canonical References

- **Spec**: `SMF://hace.uni.config.v1` — `.know/canon/specs.ail`
- **Blueprint**: `AIL://hace.uni.canon.blueprint.v1` — `.know/canon/blueprint.ail`
- **Hookpoints**: `hok://uni/config/*` — `.know/canon/hookpoint.ail`
- **FAN**: 4 features — `.know/canon/fan.ail`
- **ASI**: Integration layer — `.know/canon/asi.ail`

**END OF README**
