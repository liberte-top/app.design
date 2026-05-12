# App.Design AGENTS Guide

## Current Flow
- This repository is the **stateless visual reference** for `@liberte/svelte-components`.
- Every primitive, atom, and composition is rendered with its full variant matrix in both light and dark themes.
- The site itself has no business state; the `api/` is a stub (`/api/v1/health`) kept for symmetry with `app.smoke` and as a future extension hook.
- Container runtime is owned by the workspace `compose.yml` under the `design` profile (`design-api + design-web`); this repo does not carry its own `docker-compose.yml`.
- Public domain target: `design.liberte.top`.

## Repository Structure

```text
app.design/
├── api/                      # Rust + Axum stub (health only)
│   ├── src/
│   ├── Dockerfile
│   └── Cargo.toml
├── web/                      # Vite + Svelte showcase site
│   ├── src/                  # multi-page SPA, hash-routed
│   ├── docker/               # nginx config (proxies /api/ to design-api)
│   ├── public/favicon.svg    # canonical brand glyph
│   ├── Dockerfile
│   └── package.json
└── AGENTS.md

# Container runtime topology lives in workspace `compose.yml` (profile `design`).
```

## Role inside the design loop
- Every bump of `@liberte/svelte-components` should land here first to confirm visuals.
- Pages are flat — no auth, no API state, no localStorage beyond `liberte-theme`.
- When a token / atom / variant changes upstream, the relevant showcase row should change visibly here. Treat this site as a regression baseline.

## Runtime Parameters
Workspace `.env` (`liberte.top/.env.example`):
- `DESIGN_API_PORT`
- `DESIGN_WEB_PUBLIC_PORT`
- `DESIGN_WEB_VITE_ENV_LABEL`

## Execution Entry
- `docker compose --profile design up -d` (from workspace root)
- `cd api && cargo test --locked`
- `cd web && pnpm install --frozen-lockfile && pnpm build`

## Common Commands
- `docker compose --profile design up -d` (from workspace root)
- `docker compose --profile design ps`
- `docker compose logs -f design-api`
- `cd api && cargo test --locked`
- `cd web && pnpm build`

## Change Policy
- This module owns *no* business behavior. If you find yourself wiring up auth, persistence, or cross-page state — stop. That belongs in `service.*` or `app.smoke` / `app.registry`.
- Showcase pages should bias toward exhaustiveness, not narrative: every variant a component declares deserves to render side-by-side, even if it looks redundant.
- The api stub is intentional. Do not delete it (workspace compose / k8s expectations). Do not extend it without an explicit reason.

## Shared Package Consumption
- `web/` consumes `@liberte/svelte-components` from `https://packages.liberte.top/`.
- Repository-level npm registry mapping may be committed when it contains no auth material.
- Keep npm auth material in machine-level config or CI secrets only.
- CI and Docker builds need `packages.liberte.top` read access; pass `LIBERTE_PACKAGES_NPM_TOKEN` as a BuildKit secret rather than vendoring package sources into the image build context. Local compose gets it from the inherited machine environment; do not write registry tokens into workspace `.env`.
