# Notes on Rocket

## Important concepts

- Rust main function cannot use async by default;
- When running async you need to use `#[rocket::main]` macro to fix it;
- By replacing `#[launch]` with `#[rocket::main]` you need to add launch() to the end of the rocket function;
- You also need to add await after lauch();
- Rocket has some figments providers that overwrite default rocket::Config. Do this by creating a Rocket.toml;
- You can also overwrite these configurations using enviroment variables. An example would be "export ROCKET_PORT=3500" for using a new port;

## Request life cycle in Rocket

- Request
- Router
- Handler
  - Validate
  - Process
- Response
