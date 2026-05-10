# app.design

Public visual reference for `@liberte/svelte-components`. Lives at `design.liberte.top`.

- `web/`: Vite + Svelte 5 SPA. Multi-page hash-routed showcase that exercises every primitive, atom, and composition across all variants in both light and dark themes.
- `api/`: minimal Rust + Axum stub that exposes `/api/v1/health` only. Kept for symmetry with `app.smoke` and as a future extension hook; do not add business logic.

## Purpose

- Hosted reference manual for the design system.
- Visual regression baseline: when `@liberte/svelte-components` bumps version, this site should change visibly in the relevant section, never silently.
- Onboarding surface: a designer or new contributor opens `design.liberte.top` and immediately sees what the system can do.

## Local Flow

Container-mode loop lives in the workspace `compose.yml`; this repo does not carry its own `docker-compose.yml`.

1. From the workspace root, set `LIBERTE_PACKAGES_NPM_TOKEN` in workspace `.env` (BuildKit secret source for the web image build).
2. Bring up the design profile from workspace root: `docker compose --profile design up -d`.
3. Open `http://localhost:${DESIGN_WEB_PUBLIC_PORT}`.
4. The API is at `http://localhost:${DESIGN_API_PORT}/api/v1/health` if you need to confirm it is alive.
