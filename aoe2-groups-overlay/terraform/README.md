# Terraform: aoe2-groups-proxy infra

Manages the GCP infrastructure the proxy needs to be deployed to Cloud Run
from GitHub Actions:

- Workload Identity Federation pool + provider for GitHub OIDC
- A dedicated `github-deployer` service account + IAM bindings
- The `aoe2-groups-proxy` Artifact Registry repo
- A runtime service account (`groups-proxy@…`) the Cloud Run service runs as
- A Secret Manager secret (`aoe2-groups-proxy-sheet-ids`) holding the
  slug → sheet-ID mapping, mounted into the container
- The Cloud Run v2 service itself
- A `roles/run.invoker = allUsers` binding so the endpoint is publicly callable

It does **not** manage:

- Sheet sharing. Granting the runtime SA Viewer on each tournament Google
  Sheet is a Drive ACL action — done out-of-band by the operator.
- The real payload of the sheet-ids secret. TF creates a placeholder v1; the
  operator adds the real `[sheet_ids]` TOML as v2+ via
  `gcloud secrets versions add` (so the IDs never enter Terraform state).
- The image deployed to the service — CI rolls forward the image per commit;
  this module ignores `template.containers[0].image` drift on purpose.

## Bootstrap (run once per project)

### 1. Create the GCP project and enable APIs

```sh
gcloud projects create aoe2-streaming
gcloud config set project aoe2-streaming

gcloud services enable \
    run.googleapis.com \
    artifactregistry.googleapis.com \
    iamcredentials.googleapis.com \
    sheets.googleapis.com \
    secretmanager.googleapis.com \
    iam.googleapis.com \
    sts.googleapis.com
```

### 2. Create the Terraform state bucket

```sh
gcloud storage buckets create gs://aoe2-streaming-tf-state \
    --project=aoe2-streaming --location=europe-north1
gcloud storage buckets update gs://aoe2-streaming-tf-state --versioning
```

(The bucket itself is intentionally not managed by this module — chicken
and egg.)

### 3. Plan and apply

```sh
cd terraform
terraform init
terraform plan
terraform apply
```

Expect to see new resources for: WIF pool + provider, `github-deployer` SA,
three IAM bindings on that SA (WIF impersonation, `run.developer`,
`iam.serviceAccountUser` on the runtime SA), the `groups-proxy` runtime SA,
the AR repo with cleanup policies, the `aoe2-groups-proxy-sheet-ids` Secret
Manager secret **plus a placeholder v1**, the runtime SA's
`secretAccessor` binding on it, the Cloud Run service (mounting the secret
and booting on `:latest`, which will 404 until CI pushes the first image),
and an `allUsers` invoker binding.

### 4. Populate the sheet-ids secret (real v2)

Use the local sheet-ids.toml (gitignored copy of
[../backend/sheet-ids.example.toml](../backend/sheet-ids.example.toml)):

```sh
gcloud secrets versions add aoe2-groups-proxy-sheet-ids \
    --project=aoe2-streaming \
    --data-file=../backend/sheet-ids.toml
```

This creates v2 with the real IDs; the placeholder v1 stays where it is.
The Cloud Run service mounts `latest`, which resolves to v2 (or newer) at
the next revision creation — so the new IDs take effect on the next image
push to `main`.

To rotate an ID later: edit your local file, `gcloud secrets versions add …`,
push to trigger a new revision.

### 5. Share tournament sheets with the runtime SA

```sh
RUNTIME_SA=$(terraform output -raw runtime_sa)
echo "Share each tournament Google Sheet with: $RUNTIME_SA (Viewer)"
```

Open each sheet in [tournaments.toml](../backend/tournaments.toml), click **Share**,
add `$RUNTIME_SA` as Viewer.

### 6. Hand the WIF outputs to GitHub

```sh
gh variable set WIF_PROVIDER --body "$(terraform output -raw wif_provider)"
gh variable set DEPLOYER_SA  --body "$(terraform output -raw deployer_sa)"
gh variable set CLOUD_RUN_SERVICE --body "$(terraform output -raw service_name)"
gh variable set CLOUD_RUN_URL --body "$(terraform output -raw service_url)"
```

These are stored as **repo variables** (not secrets) — none of them are
sensitive on their own.

### 7. Trigger the first deploy

Push to `main` (or run `make publish` from a workstation that has done
`gcloud auth login`). CI builds the image, pushes it as `:${sha}` + `:latest`
to AR, and rolls a new Cloud Run revision via `gcloud run services update`.

## Day-to-day

- **Deploy code changes**: push to `main`; the `backend-deploy` workflow
  handles build + push + revision rollout.
- **Add / edit a tournament**: edit [tournaments.toml](../backend/tournaments.toml)
  and push to `main`; CI rebuilds the image with the new file and rolls the
  service to it.
- **Change infra (e.g. bump memory, add an env var)**: edit the `.tf` files,
  `terraform plan`, `terraform apply`.
- **Add a new tournament sheet**: edit tournaments.toml *and* add a line
  under `[sheet_ids]` in a fresh local copy of sheet-ids.toml. Push the
  TOML change, then `gcloud secrets versions add aoe2-groups-proxy-sheet-ids
  --data-file=./sheet-ids.toml` to push the new secret version. Share the
  sheet with the runtime SA as Viewer.
- **Rotate a sheet ID**: same as above but only the secret needs updating.
