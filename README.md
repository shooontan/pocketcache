# pocketcache

![Crates.io](https://img.shields.io/crates/v/pocketcache)
![Test](https://github.com/shooontan/pocketcache/workflows/Test/badge.svg)

A simple in-memory cache for Rust.


## Install

```toml
[dependencies]
pocketcache = "0.0.1"
```


## Usage

```rust
use pocketcache::cache::Cache;
use pocketcache::time::Expiration;

#[derive(Debug)]
struct Data {
  messages: Vec<String>,
}

fn main() {
  let expiration = Expiration::Hour(3);
  let mut cache = Cache::<Data>::new(expiration);

  // set
  cache.set(
    "fruit",
    Data {
      messages: vec![String::from("peach")],
    },
  );

  // get
  let fruit = cache.get("fruit");
  println!("{:#?}", fruit); // Some( Data { messages: { ["peach"] } } )

  // delete
  cache.delete("fruit");
  let fruit = cache.get("fruit");
  println!("{:#?}", fruit); // None
}
```

## API

### Expiration

```rust
use pocketcache::time::Expiration;

let expiration = Expiration::Second(30); // 30ces
let expiration = Expiration::Minute(5); // 5min
let expiration = Expiration::Hour(3); // 3 hour
let expiration = Expiration::Default; // 1 hour
```

### Cache

```rust
use pocketcache::cache::Cache;
use pocketcache::time::Expiration;

let mut cache = Cache::<&str>::new(Expiration::Default);
```

#### set

```rust
cache.set("fruit", "banana");
cache.set("vegetable", "carrot");
cache.set("meat", "crab");
```

#### get

```rust
let mut cache = Cache::<&str>::new(Expiration::Default);

let fruit = cache.get("fruit");
println!("{:#?}", fruit); // None

cache.set("fruit", "banana");
let fruit = cache.get("fruit");
println!("{:#?}", fruit); // Some("banana")

cache.set("fruit", "peach");
let fruit = cache.get("fruit");
println!("{:#?}", fruit); // Some("peach")

// after 1 hour...

let fruit = cache.get("fruit");
println!("{:#?}", fruit); // None
```

#### delete

```rust
cache.set("fruit", "banana");
cache.delete("fruit");

let fruit = cache.get("fruit");
println!("{:#?}", fruit); // None
```

#### clear

```rust
cache.set("fruit", "banana");
cache.set("vegetable", "carrot");
cache.set("meat", "crab");
cache.clear();

println!("{:#?}", cache.get("fruit")); // None
println!("{:#?}", cache.get("vegetable")); // None
println!("{:#?}", cache.get("meat")); // None
```
