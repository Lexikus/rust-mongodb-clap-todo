# Rust TODO Clap app

Just playing around with some libraries.

## How to run it:
```bash
# starts a mongodb and mongo-express docker container.
make mongo
# returns the created uuid which can be used in the command after.
cargo run -- add <string>
# returns the of the task when found.
cargo run -- get uuid

```