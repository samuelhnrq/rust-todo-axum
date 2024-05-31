env "local" {

  # Define the URL of the database which is managed
  # in this environment.
  url = "postgres://rust_todo:rust_todo@localhost:5433/rust_todo?sslmode=disable"

  # Define the URL of the Dev Database for this environment
  # See: https://atlasgo.io/concepts/dev-database
  dev = "docker://postgres/16/dev"
}

env {
  # Declare where the schema definition resides.
  # Also supported: ["file://multi.hcl", "file://schema.hcl"].
  src = "file://schema.hcl"
}
