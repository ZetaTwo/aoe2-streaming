# Terraform: aoe2-groups-proxy infra

Manages a single resource: the `groups-proxy@…` runtime service account.
The app itself runs on k3s (see the sibling `infrastructure` repo, Flux CD
+ GHCR + `ansible-vault`-encrypted secrets) and authenticates to the Google
Sheets API with a downloaded key from this SA — don't delete it.

Does **not** manage sheet sharing: granting the runtime SA Viewer on a
tournament Google Sheet is a Drive ACL action, done out-of-band.

## Day-to-day

- **Share a new tournament sheet with the runtime SA**:
  ```sh
  RUNTIME_SA=$(terraform output -raw runtime_sa)
  echo "Share each tournament Google Sheet with: $RUNTIME_SA (Viewer)"
  ```
  Open the sheet, click **Share**, add `$RUNTIME_SA` as Viewer.
- **Rotate the runtime SA's key**: generate a new key
  (`gcloud iam service-accounts keys create ...`), vault-encrypt it into
  `infrastructure`'s `ansible/roles/aoe2-groups-proxy/files/service-account.json`,
  `make ansible-apply` there, then delete the old key
  (`gcloud iam service-accounts keys list/delete`) once the new one's
  confirmed working.
