# Running

`cargo run` does the trick.

compose.yaml provided for convenience try `docker compose up -d db` for the db. Otherwise try `docker compose up web --build` for the whole package.

# Choices

Just was playing with SeaORM to try it out and they seem to be a one trick (RDBMS) pony so far didnt want to stress it just went safe Postgres

# Drawbacks

Rust compile times sucks, which shouldn't be much of a problem since its a one shot thing and its manageable in debug mode, sadly cuz **gtg fast** maud is rust code and therefore makes iterating on the frontend a pain. I've spit into its own cargo module to remediate but rust compiler still slow.

# Tasks

- all CRUD operations
- HTMX frontend?
- clippy not failing CI/CD
