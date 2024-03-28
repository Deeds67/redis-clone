# Redis-clone

This is a pet project that implements a subset of redis functionality.
It supports multiple parallel requests (new thread per request) and uses a thread-safe in-memory key-value store (`DashMap`).

## Running the app

```bash
cargo run
```

It's accessible using `redis-cli`:

```bash
redis-cli -p 6388 SET key my-value
redis-cli -p 6388 GET key
```

## Running the tests

```bash
cargo test
```