compose_file := "compose.dev.yml"
base_compose_cmd := "podman compose -f"

help:
  @just --list

all: (db "up") (dev "up")

db state *args: (check-state state)
  {{base_compose_cmd}} {{compose_file}} {{state}} pg-dev {{args}}

dev state *args: (check-state state)
  {{base_compose_cmd}} {{compose_file}} {{state}} run-dev {{args}}

check-state state:
  @if [[ ! "{{state}}" =~ ^(up|down)$ ]]; then echo "pilih up atau down kocak!"; exit 1; fi

cargo *args:
  cargo {{args}}

seed:
  cargo test seeder -- --ignored --nocapture
