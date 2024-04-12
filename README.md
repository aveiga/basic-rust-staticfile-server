# basic-rust-staticfile-server

Rust Microservice Archetype

## Topics covered

- [REST with Warp](https://github.com/seanmonstar/warp)
- [Error Handling with Warp](https://github.com/Rust-Web-Development/code/blob/main/ch_11/handle-errors/src/lib.rs)
- [REST Calls with reqwest](https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html)
- [Messaging using amqp-rs](https://github.com/gftea/amqprs)
- [Input Validation with Validator](https://github.com/Keats/validator). [See also](https://blog.logrocket.com/json-input-validation-in-rust-web-services/)
- [DB with SQLX](https://github.com/launchbadge/sqlx)
- [DEV DB with ???]()
- [DB Versioning with SQLX](https://docs.rs/sqlx/latest/sqlx/macro.migrate.html)
- [Authentication and Authorization using OAuth v2 with ???]()
- [Service Discovery with Eureka Client]()
- [Logging with log4rs](https://docs.rs/log4rs/latest/log4rs/)
- [Testing]()
- [API Documentation with utoipa](https://docs.rs/utoipa/latest/utoipa/)
- [Monitoring with ???]()
- [Websockets with Warp]()

## Dev setup

- Install cargo-watch: `cargo install cargo-watch`

## To cleanup

## FAQ

### How to get Keycloak to run on Docker Compose on M1 MacBooks 💻?

Quick answer:

- build the image locally (more info here: https://github.com/docker/for-mac/issues/5310)
- mount the pgdata volume to a directory below your home folder (or, preferably, in the repo folder)

### How to access the RabbitMQ Management UI?

- Go to http://localhost:15672/ using username and password: guest
