# aoe2-groups-overlay frontend

Vue 3 + Vite single-page app that renders the tournament standings overlay.
See the [repo root README](../README.md) for what this overlay does, the OBS
setup instructions, and the CSS customization classes. See
[CONTRIBUTE.md](CONTRIBUTE.md) for IDE setup and tooling recommendations.

## Project Setup

```sh
pnpm install
```

## Local Dev Data

The app needs one data source, configured via env vars in a gitignored
`.env.development` file — copy [.env.development.example](.env.development.example)
to get started (Vite loads `.env.development` automatically for `pnpm dev`):

- **Real backend** — set `VITE_STANDINGS_PROXY_URL` to a running instance of
  the [backend proxy](../backend/README.md) (e.g. `http://localhost:8080` via
  `cargo run`; see its README for setup, which needs Google Sheets
  credentials).
- **Mock data** — set `VITE_USE_MOCK_DATA="true"` to skip the backend
  entirely and render a static fixture
  ([src/mocks/standings.ts](src/mocks/standings.ts)). No backend, Google
  Sheets access, or network needed — this is the quickest path when you just
  want to tweak CSS/layout. The `tournament` query param value is ignored in
  mock mode, but you still need one present, e.g. `/?tournament=anything`.

If neither is set, `fetchTournament` throws a "not configured" error, which
surfaces in the UI as the error state.

`.env.production` (committed) points at the deployed Cloud Run backend and is
used for `pnpm build`.

## Compile and Hot-Reload for Development

```sh
pnpm dev
```

## Type-Check, Compile and Minify for Production

```sh
pnpm build
```

## Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test:unit
```

## Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```
