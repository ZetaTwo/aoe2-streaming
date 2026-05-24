variable "gcp_project" {
  description = "GCP project ID hosting the proxy."
  type        = string
  default     = "aoe2-streaming"
}

variable "gcp_region" {
  description = "Region for Artifact Registry and the Cloud Run service."
  type        = string
  default     = "europe-north1"
}

variable "github_repo" {
  description = "owner/repo of the GitHub repository allowed to deploy via WIF."
  type        = string
  default     = "ZetaTwo/aoe2-streaming"
}

variable "ar_repo_id" {
  description = "Artifact Registry repository ID."
  type        = string
  default     = "aoe2-groups-proxy"
}

variable "image_name" {
  description = "Image name inside the Artifact Registry repo."
  type        = string
  default     = "aoe2-groups-proxy"
}

variable "service_name" {
  description = "Cloud Run service name."
  type        = string
  default     = "aoe2-groups-proxy"
}

variable "runtime_sa_id" {
  description = "account_id of the runtime service account (before @project.iam.gserviceaccount.com)."
  type        = string
  default     = "groups-proxy"
}

variable "allowed_origins" {
  description = "CORS origins the proxy will accept browser requests from."
  type        = list(string)
  default     = ["https://aoe2streaming.zeta-two.com"]
}

data "google_project" "this" {}

locals {
  ar_image_base = "${var.gcp_region}-docker.pkg.dev/${var.gcp_project}/${var.ar_repo_id}/${var.image_name}"

  wif_pool_id     = "github"
  wif_provider_id = "github"

  # The principal-set that GitHub OIDC tokens from this repo land in.
  wif_principal_set = "principalSet://iam.googleapis.com/projects/${data.google_project.this.number}/locations/global/workloadIdentityPools/${local.wif_pool_id}/attribute.repository/${var.github_repo}"
}
