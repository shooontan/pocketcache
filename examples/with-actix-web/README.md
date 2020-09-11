# Actix web example


```bash
cargo run
```

```bash
curl localhost:8088/hello/pocket
# Hello pocket!

curl localhost:8088/again/pocket
# Hi pocket! It's been 5 seconds since we last met!
curl localhost:8088/again/pocket
# Hi pocket! It's been 8 seconds since we last met!

curl localhost:8088/bye/pocket
# Goodbye pocket!

curl localhost:8088/again/pocket
# Hi, you must be pocket, right?
```