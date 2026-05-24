output "wif_provider" {
  description = "Full WIF provider resource name. Set as the WIF_PROVIDER repo variable in GitHub."
  value       = google_iam_workload_identity_pool_provider.github.name
}

output "deployer_sa" {
  description = "Deployer service account email. Set as the DEPLOYER_SA repo variable in GitHub."
  value       = google_service_account.deployer.email
}

output "runtime_sa" {
  description = "Runtime service account email — share every tournament Google Sheet with this as Viewer."
  value       = google_service_account.runtime.email
}

output "service_name" {
  description = "Cloud Run service name (passed to `gcloud run services update --image=...` by CI)."
  value       = google_cloud_run_v2_service.proxy.name
}

output "service_url" {
  description = "Public URL of the Cloud Run service. Feed this into the frontend's VITE_STANDINGS_PROXY_URL."
  value       = google_cloud_run_v2_service.proxy.uri
}

output "ar_image_base" {
  description = "Fully-qualified image path the CI workflow pushes to."
  value       = local.ar_image_base
}
