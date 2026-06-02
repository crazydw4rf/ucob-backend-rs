all: dev

dev:
  podman compose -f compose.dev.yml up -d
