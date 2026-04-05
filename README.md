# Brezel

gRPC/Protobuf definitions shared by UTMM and Zashchita.

```
brezel/
├── src/
│   └── lib.rs       # Shared Rust helpers
├── common.proto     # Common shared types
├── inference.proto  # Zashchita -> ML inference batch API
└── policy.proto     # UTMM -> Zashchita runtime rule apply API
```

### PolicyService

| RPC | Description |
|---|---|
| `ReplaceRuntimeRules` | Prepare the inactive runtime-rule bank, then flip the active bank in Zashchita |
| `GetApplyStatus` | Return the last applied version and apply state |

`policy.proto` is intentionally narrower than UTMM's general policy model. It
only carries exact-match runtime rules that Zashchita can project directly into
the current eBPF denylist maps.

Zashchita applies runtime rules through a double-buffered bank flip: it fills
the inactive bank, then switches `ACTIVE_POLICY_BANK` once the new bank is
ready.

`applied_version` means "last version whose bank flip completed", not merely
"last request received".

### InferenceService

| RPC | Description |
|---|---|
| `AnalyzeFeatureBatch` | First-stage LightGBM analysis over flow features |
| `AnalyzeDpiBatch` | Second-stage Mamba DPI analysis over 5 packet samples |
| `GetModelStatus` | Return readiness and version info for both models |

## Usage

### As Git Submodule

```bash
git submodule add <brezel-repo-url> proto
```

## Related Projects

- **UTMM** - Policy management backend and source of truth
- **zashchita** - High-speed packet filtering with eBPF

## License

MIT License
