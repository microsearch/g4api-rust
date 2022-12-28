_list:
    @just --list

g4api-dev: (g4api "https://g4-dev.v1.mrcapi.net/swagger/v1/swagger.json")

g4api-local: (g4api "http://localhost:5000/swagger/v1/swagger.json")

g4api PATH:
    #!/usr/bin/env bash
    set -euxo pipefail

    VERSION=$(git describe --tags | sed -E 's/v([^-]+).*/\1/g')

    rm -rf g4api

    docker run --rm -v "${PWD}:/local" --user $(id -u):$(id -g) \
        --network="host" \
        openapitools/openapi-generator:cli-6.0.x generate \
        -i {{ PATH }} \
        -g rust \
        --additional-properties=supportAsync=false \
        --additional-properties=packageName=g4api \
        --additional-properties=packageVersion=$VERSION \
        -o /local/g4api

    cd g4api
    cargo fmt
