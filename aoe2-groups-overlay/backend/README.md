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

## tournaments.toml

The full tournament list (slug → sheet ID, bracket names, A1 group ranges)
is checked in at [tournaments.toml](tournaments.toml) and baked into the
Docker image. Adding or editing a tournament is just an edit + push to
`main`; CI rebuilds the image and rolls a new Cloud Run revision.

Adding a brand-new tournament also requires sharing its Google Sheet with
the runtime SA as **Viewer** (see [../terraform/README.md](../terraform/README.md)
step 4 for the email and the rationale).

## Local development

The proxy uses Google Application Default Credentials. Two ways to provide
them locally:

```sh
# Option A: download a service-account key (do NOT check it in).
GOOGLE_APPLICATION_CREDENTIALS=./service-account.json cargo run

# Option B: impersonate the runtime SA with your own gcloud identity.
gcloud auth application-default login \
    --impersonate-service-account groups-proxy@aoe2-streaming.iam.gserviceaccount.com
cargo run
```

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
