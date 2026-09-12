output "runtime_sa" {
  description = "Runtime service account email — share every tournament Google Sheet with this as Viewer."
  value       = google_service_account.runtime.email
}
