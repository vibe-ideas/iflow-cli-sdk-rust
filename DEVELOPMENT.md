## Release Process

This project uses automated releases to [crates.io](https://crates.io/crates/iflow-cli-sdk-rust) via GitHub Actions.

### Automated Release

1. **Version Bumping**: Update the version in `Cargo.toml` following [semantic versioning](https://semver.org/)
2. **Create Tag**: Create and push a version tag:

   ```bash
   git tag v1.2.3  # Replace with actual version
   git push origin v1.2.3
   ```

3. **Automated Publishing**: The GitHub Actions workflow will automatically:
   - Run the full test suite
   - Check code formatting and linting
   - Publish to crates.io if all checks pass

### Manual Release (if needed)

If automated release fails, you can publish manually:

```bash
# Ensure you have a crates.io account and API token
cargo login  # Enter your crates.io API token
cargo publish
```

### Setting up Automated Releases

To enable automated releases, add the following secret to your GitHub repository:

1. Go to Repository Settings → Secrets and variables → Actions
2. Add a new repository secret named `CRATES_IO_TOKEN`
3. Set the value to your crates.io API token (get it from <https://crates.io/me>)

The workflow file is located at `.github/workflows/release.yml`.
