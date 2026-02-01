# Brezel

gRPC/Protobuf definitions for UTM system.

## Structure

```
brezel/
├── common.proto    # Common types (FlowKey)
└── policy.proto    # Policy service API
```

## Services

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

# Clone with submodules
git clone --recurse-submodules <your-project-url>
```


## Related Projects

- **zashchita** - High-speed packet filtering with eBPF

## License

MIT License
