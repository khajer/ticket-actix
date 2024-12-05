# generate .env
```
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

rust link lib (postgresql)
```
export DYLD_LIBRARY_PATH="/opt/homebrew/opt/postgresql@15/lib:$DYLD_LIBRARY_PATH"
```

run server
```
cargo run --bin ticket-actix
```