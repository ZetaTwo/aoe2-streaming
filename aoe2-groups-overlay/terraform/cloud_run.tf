resource "google_cloud_run_v2_service" "proxy" {
  name     = var.service_name
  location = var.gcp_region

  # Allows TF-driven replacement (e.g. region change). Default `true` blocks
  # destroy until you flip this and re-apply.
  deletion_protection = false

  ingress = "INGRESS_TRAFFIC_ALL"

  template {
    service_account = google_service_account.runtime.email

    scaling {
      min_instance_count = 0
      max_instance_count = 4
    }

    containers {
      # Placeholder before the first CI image push. CI replaces this per-deploy
      # via `gcloud run services update --image=...`; `ignore_changes` below
      # masks the resulting drift so TF doesn't fight CI.
      image = "${local.ar_image_base}:latest"

      ports {
        container_port = 8080
      }

      resources {
        limits = {
          cpu    = "1"
          memory = "512Mi"
        }
      }

      env {
        name  = "RUST_LOG"
        value = "info"
      }
      env {
        name  = "ALLOWED_ORIGINS"
        value = join(",", var.allowed_origins)
      }
      env {
        name  = "SHEET_IDS_PATH"
        value = "/etc/aoe2-groups-proxy/sheet-ids.toml"
      }

      volume_mounts {
        name       = "sheet-ids"
        mount_path = "/etc/aoe2-groups-proxy"
      }
    }

    volumes {
      name = "sheet-ids"
      secret {
        secret = google_secret_manager_secret.sheet_ids.secret_id
        items {
          version = "latest"
          path    = "sheet-ids.toml"
        }
      }
    }
  }

  lifecycle {
    # CI owns the image. Terraform owns everything else about the service.
    ignore_changes = [
      template[0].containers[0].image,
      # gcloud writes these into the resource on every `run services update`;
      # by design they belong to whichever client touched the service last.
      client,
      client_version,
      # Service-level scaling (separate from template.scaling above) — Cloud
      # Run fills in defaults we don't actively manage.
      scaling,
    ]
  }

  depends_on = [
    google_service_account_iam_member.deployer_act_as_runtime,
    # Mount requires the secret to exist and the runtime SA to be able to read it.
    google_secret_manager_secret_iam_member.runtime_reader,
    google_secret_manager_secret_version.bootstrap,
  ]
}

# Public read endpoint — anyone can call /tournaments/:slug.
resource "google_cloud_run_v2_service_iam_member" "public" {
  project  = google_cloud_run_v2_service.proxy.project
  location = google_cloud_run_v2_service.proxy.location
  name     = google_cloud_run_v2_service.proxy.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}
