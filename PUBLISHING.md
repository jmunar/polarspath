# Publishing Guide for PolarsPath Crates

This guide explains how to publish the PolarsPath crates to crates.io.

## Prerequisites

1. **Create a crates.io account** (if you don't have one):
   - Go to https://crates.io and sign up with your GitHub account
   
2. **Get your API token**:
   - Visit https://crates.io/me
   - Click "New Token" and create a token (you can name it, e.g., "publishing")
   - Copy the token (you won't be able to see it again!), and paste it into your `.env` file

3. **Login to cargo**:
   ```bash
   cargo login <your-api-token>
   ```

## Important Notes


1. **`polars_structpath_types`** (no internal dependencies)
2. **`polars_structpath_derive`** (depends on `polars_structpath_types`)
3. **`structpath`** (depends on `polars_structpath_types` and `polars_structpath_derive`)
4. **`polars_structpath_protobuf`** (depends on `structpath`)


### Step 3: Update Dependencies and Publish `polars_structpath_derive`

After `polars_structpath_types` is published, update `polars_structpath_derive/Cargo.toml`:

```toml
[dependencies]
polars_structpath_types = { version = "0.1.0", path = "../polars_structpath_types" }  # Keep path for now
# Or after first publish cycle:
# polars_structpath_types = "0.1.0"
```

Then publish:
```bash
cd polars_structpath_derive
cargo publish --dry-run
cargo publish
```

### Step 4: Update Dependencies and Publish `structpath`

Update `structpath/Cargo.toml`:
```toml
[dependencies]
polars_structpath_derive = { version = "0.1.0", path = "../polars_structpath_derive", optional = true }
polars_structpath_types = { version = "0.1.0", path = "../polars_structpath_types" }
```

Then publish:
```bash
cd structpath
cargo publish --dry-run
cargo publish
```

### Step 5: Update Dependencies and Publish `polars_structpath_protobuf`

Update `polars_structpath_protobuf/Cargo.toml`:
```toml
[dependencies]
structpath = { version = "0.1.0", path = "../structpath", features = ["derive"] }
```

Then publish:
```bash
cd polars_structpath_protobuf
cargo publish --dry-run
cargo publish
```

### Step 6: Update Workspace Dependencies (After All Crates Published)

Once all crates are published, you can update the workspace `Cargo.toml` to use published versions:

```toml
[workspace.dependencies]
# Internal dependencies - now using published versions
polars_structpath_types = "0.1.0"
polars_structpath_derive = "0.1.0"
structpath = "0.1.0"
polars_structpath_protobuf = "0.1.0"
```

And update individual crate `Cargo.toml` files to remove path dependencies.

## Common Issues and Solutions

### Issue: "crate already exists"
- The crate name is already taken on crates.io
- Solution: Choose a different name or contact the owner

### Issue: "dependency `polars_structpath_types` is not published"
- You're trying to publish a crate before its dependencies
- Solution: Publish dependencies first (follow the order above)

### Issue: "no such file or directory" when publishing
- Missing files (like README.md) referenced in Cargo.toml
- Solution: Ensure all referenced files exist, or remove the reference

### Issue: "license file is missing"
- The LICENSE file isn't in the crate root
- Solution: Ensure LICENSE file exists in each crate directory, or use `license-file` instead

### Issue: Version conflicts
- After publishing, you need to bump versions for updates
- Solution: Update version in `Cargo.toml` before republishing

## Updating Published Crates

To publish a new version:

1. **Update version** in the crate's `Cargo.toml`:
   ```toml
   version = "0.1.1"  # or "0.2.0" for breaking changes
   ```

2. **Update dependent crates** if needed (if you changed the API)

3. **Publish in dependency order** again:
   ```bash
   cargo publish
   ```

## Verification

After publishing, verify your crates are available:

- Visit https://crates.io/crates/polars_structpath_types
- Visit https://crates.io/crates/polars_structpath_derive
- Visit https://crates.io/crates/structpath
- Visit https://crates.io/crates/polars_structpath_protobuf

## Best Practices

1. **Always use `--dry-run` first**: Test the publish process without actually publishing
2. **Test locally**: Make sure `cargo build` and `cargo test` pass before publishing
3. **Check documentation**: Ensure your README and docs are up to date
4. **Version semantics**: Follow [Semantic Versioning](https://semver.org/)
   - `0.1.0` → `0.1.1`: Patch (bug fixes)
   - `0.1.0` → `0.2.0`: Minor (new features, backward compatible)
   - `0.1.0` → `1.0.0`: Major (breaking changes)

## Additional Resources

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Crates.io Documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)

