##########################
# Runtime service account
##########################

# The app (running on k3s, see /infrastructure) authenticates to the Sheets
# API with a downloaded key from this SA. Do NOT delete it — the running
# deployment depends on a key already issued from it. Sheets access itself
# is granted by sharing each tournament sheet with this SA's email as
# Viewer — a Drive ACL action, not a GCP IAM role, so it isn't in Terraform.
resource "google_service_account" "runtime" {
  account_id   = var.runtime_sa_id
  display_name = "aoe2 groups proxy runtime"
  description  = "Runtime identity for the aoe2-groups-proxy app (Sheets API access)"
}
