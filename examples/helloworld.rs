extern crate couchbase;

use couchbase::Cluster;

fn main() {
    // Open the Cluster Reference
    let mut cluster = Cluster::new("127.0.0.1");

    // Open the Bucket
    let bucket = cluster.open_bucket("beer-sample", "");

    // Print the bucket name of Ok, if Err print why
    match bucket {
        Ok(b) => println!("Connected to bucket: {}", b.name()),
        Err(e) => println!("Could not connect to bucket - cause: {}", e),
    }

    // when cluster goes out of scope, calls "close" on all buckets it owns.
}
