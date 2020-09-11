use pocketcache::cache::Cache;
use pocketcache::time::Expiration;
use rand;

fn main() {
    let mut cache = Cache::<u8>::new(Expiration::Default);

    for i in 1..6 {
        let data = rand::random::<u8>();
        cache.set(&format!("{}", i), data);
        println!("set {}: {}", i, data);
    }

    for i in 1..6 {
        if i % 2 == 0 {
            cache.delete(&format!("{}", i));
        }
        let data = cache.get(&format!("{}", i));
        println!("get {}: {:?}", i, data);
    }

    cache.clear();

    for i in 1..6 {
        let data = cache.get(&format!("{}", i));
        println!("afeter clear {}: {:?}", i, data);
    }
}
