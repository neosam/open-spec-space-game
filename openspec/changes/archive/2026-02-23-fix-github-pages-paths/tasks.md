## 1. Fix GitHub Actions Workflow

- [x] 1.1 Add `actions/configure-pages@v5` step with id `pages` to the build job, before the trunk build step
- [x] 1.2 Update `trunk build --release` command to include `--public-url "${{ steps.pages.outputs.base_path }}/"`
- [x] 1.3 Add `pages: write` and `id-token: write` permissions if not already present (verify existing permissions are sufficient for configure-pages)
