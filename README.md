# Hyper Tarot (under rebranding)

Live at https://rust-todo.fly.dev/

First and foremost an Rust + HTMX playground to see for myself what the buzz is all about.

# Architecture
~Webscale ™️~

- '**V**'iew
  - [**HTMX**](https://htmx.org/): I'm not doing MVC for no reason, I'm not even old enough to be nostalgic about it, Web Frameworks come and go with the wind but [Hypermedia.Systems](https://hypermedia.systems/) is too good not to be put to the test.
  - [__hyperscript_](https://hyperscript.org/): Its already HTMX, lets drink that [bigskysoftware](https://github.com/bigskysoftware)'s kool-aid
  - [Maud](https://github.com/lambda-fairy/maud): Templating engines in rust are still 'greenfield' so anything goes really. Given the _carte blanche_ I took the liberty to [*gtg fast*](https://github.com/rosetta-rs/template-benchmarks-rs?tab=readme-ov-file#results), maud is basically a fancy macro with a DSL that compiles down to actual rust code.
- '**M**'odel
  - [SeaORM](https://github.com/SeaQL/sea-orm): Its very powerful ORM. Very powerful and yet sleek API. It opts to depend on code generation/build-step rather than only macro magic. It also tries to be a one stop shop with migrations but these didnt work so well so I also added...
  - [Liquibase](https://github.com/liquibase/liquibase): for dabatabase migration, you can't beat 20+ years of java tooling (maybe), but the cli still manages works great standalone.
- '**C**'ontroller
  - [Axum](https://github.com/tokio-rs/axum) - My edgy-ness ran out by here, it goes fast enough too and its a thin wrapper around tokio, which I wanted to learn more about
- Fully agnostic OAuth2 OpenID authentication
  - Rust oauth2 libs all sucks didn't expect this to took this long

# Deployment
- Fly.io - Around the world, free tier, deploys containers. Serverless, WebScale.
- Neon.tech - Initially wanted to checkout the branching workflow, kept just for the free tier, also Serverless, WebScale.
- Auth0 - Big free tier and Oauth2 compliant, also Serverless, WebScale.

# Running

`cargo run` does the trick.

compose.yaml provided for convenience and local development try `docker compose up -d keycloak` for a local oauth2 (keycloak) + postgres DB. Otherwise try `docker compose up web --build` for the whole package.

# Drawbacks

Rust compile times sucks, which shouldn't be much of a problem since its a one shot thing and its manageable in debug mode, sadly cuz **gtg fast** maud is rust code and therefore makes iterating on the frontend a pain. I've spit into its own cargo module to remediate but rust compiler still slow.
