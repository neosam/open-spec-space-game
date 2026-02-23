## Why

The game deployed to GitHub Pages returns 404 errors for all assets. When hosted at `username.github.io/open-spec-space-game/`, trunk generates asset URLs without the repository subpath prefix (e.g., `/spacegame-*.js` instead of `/open-spec-space-game/spacegame-*.js`).

## What Changes

- Add `actions/configure-pages@v5` step to the GitHub Actions workflow to detect the repository's base path
- Pass the detected base path to `trunk build --release --public-url` so all generated asset URLs include the correct prefix

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `github-pages-deploy`: Asset URLs must include the repository subpath so the game loads correctly when hosted under a subpath on GitHub Pages

## Impact

- `.github/workflows/deploy.yml`: Add configure-pages step and update trunk build command
- No game code changes
- No local build changes
