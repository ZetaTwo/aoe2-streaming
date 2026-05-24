# Secret holding the slug → sheet-ID mapping consumed by the proxy at startup.
# The real payload is added out-of-band so it never lands in Terraform state:
#
#   gcloud secrets versions add aoe2-groups-proxy-sheet-ids \
#       --project=aoe2-streaming --data-file=./sheet-ids.toml
#
# Format (TOML) — see backend/sheet-ids.example.toml for the template:
#   [sheet_ids]
#   <slug> = "<google-sheets-document-id>"
#   …
resource "google_secret_manager_secret" "sheet_ids" {
  secret_id = var.sheet_ids_secret_id

  replication {
    auto {}
  }
}

# Placeholder v1. Cloud Run validates at create-time that the secret version
# referenced by a volume mount exists; without v1 the service would refuse to
# come up. The real config is added later as v2+, and the `latest` mount
# resolves to whichever version is newest when a revision is created.
resource "google_secret_manager_secret_version" "bootstrap" {
  secret      = google_secret_manager_secret.sheet_ids.id
  secret_data = "# placeholder created by Terraform — populate via 'gcloud secrets versions add'\n[sheet_ids]\n"

  lifecycle {
    # Don't tempt anyone into editing this; real values arrive as v2+.
    ignore_changes = [secret_data]
  }
}

resource "google_secret_manager_secret_iam_member" "runtime_reader" {
  project   = google_secret_manager_secret.sheet_ids.project
  secret_id = google_secret_manager_secret.sheet_ids.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.runtime.email}"
}
