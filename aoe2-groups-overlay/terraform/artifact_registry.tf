resource "google_artifact_registry_repository" "proxy" {
  location      = var.gcp_region
  repository_id = var.ar_repo_id
  format        = "DOCKER"

  docker_config {
    immutable_tags = false
  }

  cleanup_policy_dry_run = false

  # Same cleanup pattern as the sibling aoe2-tournament-bot repo: a sweeping
  # DELETE policy plus KEEP exemptions for the latest 5 versions and the
  # buildcache tag. KEEP takes precedence over DELETE.
  cleanup_policies {
    id     = "delete-all"
    action = "DELETE"
    condition {
      tag_state = "ANY"
    }
  }

  cleanup_policies {
    id     = "keep-last-5"
    action = "KEEP"
    most_recent_versions {
      keep_count = 5
    }
  }

  cleanup_policies {
    id     = "keep-buildcache"
    action = "KEEP"
    condition {
      tag_state    = "TAGGED"
      tag_prefixes = ["buildcache"]
    }
  }
}

resource "google_artifact_registry_repository_iam_member" "deployer_writer" {
  project    = google_artifact_registry_repository.proxy.project
  location   = google_artifact_registry_repository.proxy.location
  repository = google_artifact_registry_repository.proxy.name
  role       = "roles/artifactregistry.writer"
  member     = "serviceAccount:${google_service_account.deployer.email}"
}
