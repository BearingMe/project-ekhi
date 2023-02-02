# Notes on Rocket

## Important concepts

- Rust main function cannot use async by default;
- When running async you need to use `#[rocket::main]` macro to fix it;
- By replacing `#[launch]` with `#[rocket::main]` you need to add launch() to the end of the rocket function;
- You also need to add await after lauch().

## Request life cycle in Rocket

- Request
- Router
- Handler
  - Validate
  - Process
- Response
