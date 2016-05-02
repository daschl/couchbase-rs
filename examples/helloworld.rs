extern crate couchbase;

use couchbase::Cluster;

fn main() {
    // Open the Cluster Reference
    let mut cluster = Cluster::new("127.0.0.1");

    // Open the Bucket
    let bucket = cluster.open_bucket("beer-sample", "");

    match bucket {
        Ok(b) => println!("{}", b.name()),
        Err(e) => println!("Could not connect - cause: {}", e),
    }
}
