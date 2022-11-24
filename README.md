# actix-rest-api
Trying simple rest-api using actixweb, serde, postgresql and sqlx

Database migrations were made with dbmate

```
dbmate --url DATABASE_URL up
```

while compile, you should set your 
DATABASE_URL
environment variable in 
```
.cargo/config.toml 
```

you may create dir with file in current dir using template:

```
[env]
DATABASE_URL="postgres://user:pass@localhost:5432/todolist?sslmode=disable"
```


run with
```
cargo run
```

4 types of requests
```
#[get("/todolist/entries")] // to get all entries
```
```
#[post("/todolist/entries/{id}")] // with JSON {"title"="{something}", "date"=1669298264} to add entries
```
```
#[put("/todolist/entries/{id}")] // with JSON {"title"="{something}"} to update enties
```
```
#[delete("/todolist/entries/{id}")] // to delete entries
```
