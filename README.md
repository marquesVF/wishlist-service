# 📜️ Wishlist Microservice

This is a showcase of a microservice written in Rust. The microservice exposes:
-  An API for wihslists management;
-  A swagger page to help testing and reasoning about the API;
-  An OpenAPI JSON specification that can be used by a frontend or another service to auto-generate the types of data used by this microservice; 

## ⚙️ Local development setup:

1. Install [rustup](https://www.rust-lang.org/tools/install) for rust version management;
2. (Optional) Install [cargo-watch](https://crates.io/crates/cargo-watch) for live reloading of the server: run `cargo install cargo-watch` to install it;
3. Create an `.env` file from the `env.template`. You need to set the PostgreSQL database URL there;
4. Run `cargo watch -x run` to start the server, and automatically apply your changes during development;
5. This project uses [sqlx](https://crates.io/crates/sqlx) for database connection and compile-time validation of SQL queries. Therefore, you need to have the database available during compilation;

### Setup database

This project uses PostgreSQL as a database. We suggest running it in a docker container;

#### Run Postgres as a docker container (optional)
1. Make sure that you have docker installed, and that you are inside this project root folder;

2. Run `docker build database -t wishlist-db` to build an image for local development with testing data;

> 💡 To build a Postgres docker image with the tables, but without testing data for the production environment, you can run the following: `docker build database -t wishlist-db --build-arg ENV=production`

3. Run `docker run -e POSTGRES_PASSWORD=<YOUR_PW> -p 5432:5432 wishlist-db` to run the Postgres container with port forwarding;

4. Since Postgres is running on docker, changes in the database do not persist automatically. Nevertheless, you can follow these steps to persist your data:

    4.1. Create a docker volume: `docker volume create postgres_data`;

    4.2. When running Postgres, pass the created volume as an argument: `docker run -e POSTGRES_PASSWORD=<YOUR_PW> -p 5432:5432 v postgres_data:/var/lib/postgresql/data wishlist-db`;

