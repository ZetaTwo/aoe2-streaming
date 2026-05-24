terraform {
  backend "gcs" {
    bucket = "aoe2-streaming-tf-state"
    prefix = "aoe2-groups-proxy"
  }
}
