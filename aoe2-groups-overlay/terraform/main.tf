variable "gcp_project" {
  description = "GCP project ID hosting the proxy."
  type        = string
  default     = "aoe2-streaming"
}

variable "gcp_region" {
  description = "Region for GCP resources."
  type        = string
  default     = "europe-north1"
}

variable "runtime_sa_id" {
  description = "account_id of the runtime service account (before @project.iam.gserviceaccount.com)."
  type        = string
  default     = "groups-proxy"
}
