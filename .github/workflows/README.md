# GitHub Actions Workflows

This directory contains the CI/CD workflows for the P2P Energy Trading platform.

## Workflows Overview

| Workflow | Purpose | Triggers |
|----------|---------|----------|
| `contracts.yml` | Smart contract CI/CD | Changes to `contracts/**` |
| `docker.yml` | Docker services CI/CD | Changes to `docker/**` |
| `frontend.yml` | Frontend CI/CD | Changes to `frontend/**` |
| `main.yml` | Main orchestration pipeline | All changes |

## Smart Contracts Workflow (`contracts.yml`)

### Performance Optimizations

The contracts workflow includes several performance optimizations:

#### 🚀 Caching Strategy
- **Cargo Registry**: Caches Rust dependencies and compilation artifacts
- **Tool Binaries**: Caches `cargo-contract` and `cargo-audit` binaries
- **Source-Aware**: Cache keys include source file hashes for precision
- **Multi-Level Fallbacks**: Progressive cache degradation for reliability

#### ⚡ sccache Integration
The workflow attempts to use [sccache](https://github.com/mozilla/sccache) for distributed compilation caching:

- **Automatic Fallback**: If sccache fails to start (e.g., GitHub cache service issues), builds continue normally
- **Error Resilience**: Network issues or service outages don't break the build
- **Statistics Reporting**: Shows cache hit rates when sccache is active

#### 🔧 Expected Performance
- **First Build**: ~15-20% faster (tool caching)
- **Subsequent Builds**: ~60-80% faster (full cache hits)
- **Source Changes**: ~40-60% faster (dependency cache reuse)

### Troubleshooting

#### sccache Issues
If you see errors like:
```
sccache: error: Server startup failed: cache storage failed to read
```

This indicates a temporary GitHub Actions cache service issue. The workflow will:
1. ✅ Automatically fall back to standard compilation
2. ✅ Continue the build without sccache
3. ✅ Display helpful error messages

No action needed - the build will complete successfully.

#### Cache Invalidation
Caches are automatically invalidated when:
- `Cargo.lock` changes (dependency updates)
- Source files in relevant directories change
- Tool versions are updated

#### Manual Cache Management
To clear caches manually:
1. Go to your repository → Actions → Caches
2. Delete specific cache entries
3. Re-run workflows to rebuild caches

## Best Practices

### For Contributors
- Keep commits focused to maximize cache reuse
- Update dependencies in separate commits when possible
- Test locally before pushing to reduce CI cycles

### For Maintainers
- Monitor cache hit rates in workflow logs
- Update tool versions periodically
- Review cache storage usage in repository settings

## Monitoring

Each workflow provides detailed logging:
- Cache hit/miss statistics
- Compilation time metrics
- sccache performance data (when available)
- Build artifact sizes

Check the "📊 sccache statistics" steps in workflow runs for performance insights.
