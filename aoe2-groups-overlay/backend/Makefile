default: aoe2-groups-proxy.hash

# Run the image locally with a service-account key bind-mounted. Useful for
# verifying that the Sheets API call works against real tournament sheets.
test: aoe2-groups-proxy.hash
	docker run \
		-v ${PWD}/service-account.json:/app/service-account.json:ro \
		-e GOOGLE_APPLICATION_CREDENTIALS=/app/service-account.json \
		-e ALLOWED_ORIGINS=http://localhost:5173,https://aoe2streaming.zeta-two.com \
		-e PORT=8080 \
		-p 8080:8080 \
		--rm -it aoe2-groups-proxy:latest

publish: aoe2-groups-proxy.hash
	gcloud auth configure-docker europe-north1-docker.pkg.dev
	docker tag aoe2-groups-proxy:latest europe-north1-docker.pkg.dev/aoe2-streaming/aoe2-groups-proxy/aoe2-groups-proxy:latest
	docker push europe-north1-docker.pkg.dev/aoe2-streaming/aoe2-groups-proxy/aoe2-groups-proxy:latest

aoe2-groups-proxy.hash: Dockerfile Cargo.toml Cargo.lock tournaments.toml $(shell find src -type f)
	docker buildx build --iidfile $@ --tag aoe2-groups-proxy .

.PHONY: default test publish
