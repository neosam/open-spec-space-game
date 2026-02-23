## Context

The game is deployed to GitHub Pages via a GitHub Actions workflow. GitHub Pages hosts the site at `username.github.io/repo-name/`, meaning all assets live under a subpath. Trunk generates asset URLs as absolute paths from root (e.g., `/spacegame-*.js`), which results in 404 errors because the browser requests them from the domain root instead of the repo subpath.

## Goals / Non-Goals

**Goals:**
- Asset URLs in the built WASM output include the correct repository subpath prefix
- The fix works automatically for any repository name (not hardcoded)

**Non-Goals:**
- Custom domain setup
- Changes to local development builds
- Game code changes

## Decisions

**Use `actions/configure-pages@v5` to detect the base path dynamically**

Rather than hardcoding the repository name in the workflow, use the official `actions/configure-pages@v5` action which outputs the base path. This keeps the workflow portable across forks and repository renames.

Alternative considered: Hardcoding `--public-url /open-spec-space-game/` — rejected because it would break on forks or renames.

**Pass base path to `trunk build --public-url`**

Trunk's `--public-url` flag prefixes all generated asset URLs with the given path. Using `${{ steps.pages.outputs.base_path }}/` from configure-pages ensures correct paths.

## Risks / Trade-offs

- [Trailing slash sensitivity] The `--public-url` value needs the correct trailing slash to avoid double or missing slashes. → Mitigation: `configure-pages` outputs the base path without trailing slash, so we append `/` explicitly.
