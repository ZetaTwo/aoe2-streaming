# aoe2-groups-proxy

Tiny Rust+Axum HTTP proxy that fetches tournament standings from Google
Sheets on behalf of the [groups overlay](../frontend) frontend. Runs on
Cloud Run, authenticates to Sheets as a service account, and lets the
frontend drop its public API key.

## API

- `GET /healthz` → `OK`
- `GET /tournaments/:slug` → `{ "brackets": [BracketStanding, …] }`

`BracketStanding` matches the frontend types in
[`standings.ts`](../frontend/src/stores/standings.ts):

```jsonc
{
  "brackets": [
    {
      "name": "Champions",
      "groups": [
        {
          "name": "Group A",
          "players": [
            { "rank": 1, "name": "TheViper",
              "setsWon": 2, "setsLost": 0, "mapsWon": 4, "mapsLost": 1 }
          ]
        }
      ]
    }
  ]
}
```

Status codes:
- `404` — slug not in [tournaments.toml](tournaments.toml).
- `502` — Sheets API call failed (auth, transient, or the sheet isn't
  shared with the runtime SA).

## Config split

Two files feed the proxy at startup:

- [tournaments.toml](tournaments.toml) — slugs, bracket names, A1 group
  ranges. Checked into git, baked into the Docker image. Editing it
  requires a new image (CI rebuilds on push to `main`).
- `sheet-ids.toml` — `[sheet_ids]` table mapping each slug to its Google
  Sheets document ID. **Not in git.** In production it's held in Secret
  Manager (secret `aoe2-groups-proxy-sheet-ids`) and mounted by Cloud Run
  at `/etc/aoe2-groups-proxy/sheet-ids.toml`. Locally, copy
  [sheet-ids.example.toml](sheet-ids.example.toml) and fill in real IDs.

The split exists because the upstream tournament sheets are configured with
"edit with link" sharing — knowing an ID is enough to vandalize the data.
Keeping IDs out of git removes one disclosure path; the runtime SA's
implicit access (the SA is added as Viewer separately, see
[../terraform/README.md](../terraform/README.md)) is what authorizes reads.

Adding or editing a tournament:
1. Edit `tournaments.toml` (slug + brackets); push to `main`.
2. Edit your local `sheet-ids.toml`; push the new version to Secret Manager:
   `gcloud secrets versions add aoe2-groups-proxy-sheet-ids --data-file=./sheet-ids.toml`.
3. Share the sheet with the runtime SA as Viewer.

Adding a *placeholder* tournament (e.g. before its sheet is ready) only
needs step 1 with an empty `brackets = []`. The handler short-circuits to
`{"brackets":[]}` without consulting Sheets, so no ID is required yet.

## Local development

The proxy uses Google Application Default Credentials. Two ways to provide
them locally:

```sh
# Option A: download a service-account key (do NOT check it in).
GOOGLE_APPLICATION_CREDENTIALS=./service-account.json \
    SHEET_IDS_PATH=./sheet-ids.toml cargo run

# Option B: impersonate the runtime SA with your own gcloud identity.
gcloud auth application-default login \
    --impersonate-service-account groups-proxy@aoe2-streaming.iam.gserviceaccount.com
SHEET_IDS_PATH=./sheet-ids.toml cargo run
```

(`./sheet-ids.toml` is a gitignored copy of
[sheet-ids.example.toml](sheet-ids.example.toml) with real IDs filled in.)

Then:
```sh
curl http://localhost:8080/healthz                # → OK
curl http://localhost:8080/tournaments/ttlc2 | jq  # → {"brackets": …}
```

`tests`:
```sh
cargo test           # parse + config unit tests
cargo clippy --all-targets --tests -- -D warnings
cargo fmt --check
```

## Running in Docker

```sh
make                  # builds the image
make test             # runs locally with ./service-account.json mounted
```

`make test` exposes port 8080 and points the container at a SA key file in
the project root.

## Deployment

Pushed to `main` is enough — the
[backend-deploy workflow](../../.github/workflows/backend-deploy.yml) builds
the image, pushes `:${sha}` + `:latest` to Artifact Registry, and runs
`gcloud run services update` to roll the new image into the existing
service. The Cloud Run service itself (and all surrounding IAM) is managed
by Terraform — see [../terraform/README.md](../terraform/README.md) for the
one-time bootstrap.

## Environment variables

Read at startup:

| Var | Purpose | Default |
|---|---|---|
| `PORT` | TCP port to listen on (Cloud Run injects this). | `8080` |
| `ALLOWED_ORIGINS` | Comma-separated CORS origins. Empty → `*`. | empty |
| `RUST_LOG` | Tracing env filter. | `info` |
| `TOURNAMENTS_PATH` | Path to tournaments.toml. | `tournaments.toml` |
| `SHEET_IDS_PATH` | Path to sheet-ids.toml (mounted from Secret Manager in prod). | `/etc/aoe2-groups-proxy/sheet-ids.toml` |
| `CONFIG_PATH` | Optional extra TOML config file. | `config.toml` (skipped if missing) |
| `GOOGLE_APPLICATION_CREDENTIALS` | Service-account key path. Unused on Cloud Run (metadata server takes over). | unset |

The optional `config.toml` accepts a `[server]` section to override the
above without setting env vars:

```toml
[server]
bind_addr = "0.0.0.0"
port = 8080
allowed_origins = ["http://localhost:5173", "https://aoe2streaming.zeta-two.com"]
```
