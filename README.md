# Intro

A Rust-based web application leveraging Actix Web for fast and scalable web
services, MongoDB for database storage, and Shaku for dependency injection.

# Features

- ✅ MongoDB integration for a robust database layer.
- ✅ MVC structure with Dependency Injection-like components (repository, service,
controller).
- ✅ Authentication baked right in for secure app development.
= ✅ Fully tested—because quality matters.

Hit the ⭐️ if you like it.

# Install MongoDb

```shell 
    docker compose up
```

# Start/Watch

```shell 
    cargo watch -q -c -w src/ -x run
```

# Testing

Run the following command to test:

```shell 
    cargo watch -w tests -x "test -- --nocapture"
```

Http Test

```shell
    cargo watch -q -c -w tests/ -x "test  -q test_create_user -- --nocapture"
```

or

```shell
    cargo test 
```
