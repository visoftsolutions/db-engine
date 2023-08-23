# db-engine

## Development

# Create env files to start development

#### **`.cargo/config.toml`**

```
[env]
DB_USERNAME="surrealuser"
DB_PASSWORD="surrealp4ssword"
DB_WS="localhost:8000"
```

#### **`.env`**

```
DB_USERNAME="surrealuser"
DB_PASSWORD="surrealp4ssword"
```

# How to run application

```shell
docker compose up
cargo run
```
