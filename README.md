# Brezel

gRPC/Protobuf definitions for UTM system.

```
brezel/
├── src/
│   └── lib.rs      # IP address conversion functions
├── common.proto    # Common types (FlowKey)
└── policy.proto    # Policy service API
```

### PolicyService

| RPC | Description |
|---|---|
| `WatchPolicies` | Real-time policy change notifications (Server Stream) |
| `AddBlacklist` | Add blacklist entry |
| `UpdateBlacklist` | Update blacklist entry |
| `RemoveBlacklist` | Remove blacklist entry |

## Usage

### As Git Submodule

```bash
# Add to your project
git submodule add <brezel-repo-url> proto

## Related Projects

- **zashchita** - High-speed packet filtering with eBPF

## License

MIT License
