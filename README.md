# Brezel

gRPC/Protobuf definitions shared by UTMM and Zashchita.

```
brezel/
├── src/
│   └── lib.rs      # Shared Rust helpers
├── common.proto    # Common shared types
├── inference.proto # Zashchita -> ML inference batch API
└── policy.proto    # UTMM -> Zashchita policy apply API
```

### PolicyService

| RPC | Description |
|---|---|
| `ReplacePolicySet` | Replace the full active policy set in Zashchita |
| `GetApplyStatus` | Return the last applied version and apply state |

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
