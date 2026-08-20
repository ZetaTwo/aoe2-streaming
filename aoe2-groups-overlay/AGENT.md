# aoe2-groups-overlay — agent notes

OBS/XSplit browser-source overlay showing AoE2 tournament group standings.
See [README.md](README.md) for the product description.

## Layout

- [frontend/](frontend/) — Vue 3 + Vite + Pinia SPA. This is where almost
  all UI/CSS work happens. See [frontend/README.md](frontend/README.md) for
  the local dev-data setup (real backend vs. `VITE_USE_MOCK_DATA`).
- [backend/](backend/) — Rust/Axum proxy that fetches standings from Google
  Sheets. See [backend/README.md](backend/README.md).
- [terraform/](terraform/) — Cloud Run + IAM for the deployed backend. See
  [terraform/README.md](terraform/README.md).

Frontend and backend are independent: for CSS/layout/markup work you almost
never need the backend running — use mock data instead (below).

## Frontend setup

```sh
cd frontend
pnpm install
cp .env.development.example .env.development   # gitignored; pick backend or mock mode
pnpm dev                                        # http://localhost:5173
```

`VITE_USE_MOCK_DATA="true"` in `.env.development` renders a static fixture
([frontend/src/mocks/standings.ts](frontend/src/mocks/standings.ts)) instead
of hitting the backend — use this for CSS/layout work. It covers brackets
with 1, 2, 3, and 4 groups so odd/even column-wrapping behavior is
exercised. Load any bracket via query params, e.g.
`http://localhost:5173/?tournament=x&bracket=Pikemen` (the `tournament`
value is ignored in mock mode, but one must be present or nothing fetches).

## Validation

Run from `frontend/`:

```sh
pnpm type-check    # vue-tsc
pnpm lint          # oxlint + eslint, both --fix
pnpm test:unit run  # vitest
pnpm build-only     # prod build; confirms it compiles and tree-shakes cleanly
```

`pnpm build-only` also matters for anything env-gated (like the mock data
path): grep the built bundle to confirm dev-only code didn't leak into prod,
e.g. `grep -c mockBrackets dist/assets/index-*.js` should be `0` since
`.env.production` doesn't set `VITE_USE_MOCK_DATA`. Remove `dist/` after
(it's gitignored, but don't leave build artifacts lying around).

### Visual verification for UI/CSS changes

Type-check/lint/tests don't catch layout regressions — always render the
actual page and look at it before calling a CSS/markup change done. There's
no project-specific browser-driving skill here yet, so the pattern is: spin
up the dev server with mock data, drive headless Chromium via Playwright,
screenshot, and read the image.

One-time setup (Playwright's browser binary isn't part of `node_modules`
here and needs its own install; installing it *inside* `frontend/` would
touch the committed lockfile, so do it in a scratch dir instead):

```sh
mkdir -p /tmp/pw-scratch && cd /tmp/pw-scratch
npm init -y && npm install playwright
npx playwright install chromium   # ~300MB download, skip --with-deps (needs root)
```

Per change:

```sh
cd frontend
lsof -ti:5173 -sTCP:LISTEN | xargs -r kill   # free the port if a stale server is running
pnpm dev >/tmp/vite-dev.log 2>&1 & disown
timeout 30 bash -c 'until curl -sf http://localhost:5173 >/dev/null; do sleep 1; done'
```

Then a small Playwright script (adapt path/URLs per change) — navigate,
wait for a real element (not a raw `sleep`), screenshot, and check
`console --errors`-equivalent (`page.on('console'/'pageerror')`):

```js
// /tmp/pw-scratch/shot.js
const { chromium } = require('playwright')
async function main() {
  const browser = await chromium.launch({ args: ['--no-sandbox'] })
  const page = await (await browser.newContext({ viewport: { width: 1920, height: 1080 } })).newPage()
  const errors = []
  page.on('console', (m) => m.type() === 'error' && errors.push(m.text()))
  page.on('pageerror', (e) => errors.push(String(e)))

  await page.goto('http://localhost:5173/?tournament=x&bracket=Pikemen', { waitUntil: 'networkidle' })
  await page.waitForSelector('.bracket-title')
  await page.screenshot({ path: '/tmp/pw-scratch/out.png', fullPage: true })

  console.log('errors:', JSON.stringify(errors))
  await browser.close()
}
main()
```

```sh
node /tmp/pw-scratch/shot.js
```

Read the resulting PNG with the Read tool — don't just check the script
exited 0. Cover both an odd- and even-group-count bracket
(`bracket=Knights` = 3 groups, `bracket=Pikemen` = 4 groups) when the change
touches the groups grid, since wrapping behavior differs.

Afterwards: `lsof -ti:5173 -sTCP:LISTEN | xargs -r kill` to stop the dev
server.
