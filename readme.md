# Devhub Cache API

This repository leverages PostgreSQL as a caching layer to reduce DevHub's RPC calls to a rate of 1 per second. The API is built using Rust's Rocket framework and deployed on Fly.io.

## Develop

```sh
cargo run
```

or

```sh
cargo watch -q -c -w src/ -x 'run '
```

### SQLx Postgres
---
[More information](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)

### Create and run migrations

*Github codespace*: If the database server is not running, you can start it by typing `sudo service postgresql start`.

```bash
sqlx migrate add <name>
```

Creates a new file in `migrations/<timestamp>-<name>.sql`. Add your database schema changes to
this new file.

---

```bash
sqlx migrate run
```

Compares the migration history of the running database against the `migrations/` folder and runs
any scripts that are still pending.

## Deploy

Until the ci/cd.yml is fixed the only way to deploy is with the fly cli.

Install for linux:
```sh
curl -L https://fly.io/install.sh | sh
```

Install for Mac:
```sh
brew install flyctl
```

Then 
```
fly deploy -c fly.*.toml
```





