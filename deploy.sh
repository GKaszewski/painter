#!/usr/bin/env bash
set -euo pipefail

REGISTRY="registry.gabrielkaszewski.dev"
REPO="painter"

tag="latest"

while [[ $# -gt 0 ]]; do
  case $1 in
    --tag) tag="$2"; shift 2 ;;
    *) echo "usage: $0 [--tag T]" >&2; exit 1 ;;
  esac
done

image="${REGISTRY}/${REPO}:${tag}"

echo "building ${image}"
docker buildx build --platform linux/amd64 \
  -t "$image" --push .
echo "pushed $image"
