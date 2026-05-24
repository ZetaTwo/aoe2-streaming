############################################
# Workload Identity Federation (GitHub → GCP)
############################################

resource "google_iam_workload_identity_pool" "github" {
  workload_identity_pool_id = local.wif_pool_id
  display_name              = "GitHub Actions"
  description               = "Pool for GitHub Actions OIDC tokens"
}

resource "google_iam_workload_identity_pool_provider" "github" {
  workload_identity_pool_id          = google_iam_workload_identity_pool.github.workload_identity_pool_id
  workload_identity_pool_provider_id = local.wif_provider_id
  display_name                       = "GitHub OIDC"

  oidc {
    issuer_uri = "https://token.actions.githubusercontent.com"
  }

  attribute_mapping = {
    "google.subject"       = "assertion.sub"
    "attribute.repository" = "assertion.repository"
    "attribute.ref"        = "assertion.ref"
  }

  # Only tokens from this specific repo are accepted.
  attribute_condition = "assertion.repository == '${var.github_repo}'"
}

############################
# Deployer service account
############################

resource "google_service_account" "deployer" {
  account_id   = "github-deployer"
  display_name = "GitHub Actions deployer"
  description  = "Identity assumed by the CI workflow via WIF"
}

# Let GitHub Actions tokens from this repo impersonate the deployer SA.
resource "google_service_account_iam_member" "deployer_wif" {
  service_account_id = google_service_account.deployer.name
  role               = "roles/iam.workloadIdentityUser"
  member             = local.wif_principal_set
}

# Project-level role: manage Cloud Run revisions.
resource "google_project_iam_member" "deployer_run_developer" {
  project = var.gcp_project
  role    = "roles/run.developer"
  member  = "serviceAccount:${google_service_account.deployer.email}"
}

# Allow the deployer to attach the runtime SA to a Cloud Run revision.
resource "google_service_account_iam_member" "deployer_act_as_runtime" {
  service_account_id = google_service_account.runtime.name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.deployer.email}"
}

##########################
# Runtime service account
##########################

# The Cloud Run service runs as this SA. To grant it Sheets API access, the
# operator must share each tournament Google Sheet with this SA's email as
# Viewer — that's a Drive ACL action, not a GCP IAM role, so it doesn't
# appear in Terraform.
resource "google_service_account" "runtime" {
  account_id   = var.runtime_sa_id
  display_name = "aoe2 groups proxy runtime"
  description  = "Runtime identity for the Cloud Run service that reads tournament sheets"
}
