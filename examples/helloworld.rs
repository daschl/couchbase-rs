extern crate couchbase;

use couchbase::Cluster;

fn main() {
    // Open the Cluster Reference
    let mut cluster = Cluster::new("127.0.0.1");

    // Open the Bucket
    let bucket = cluster.open_bucket("beer-sample", "").expect("Could not connect to bucket!");

    // Load the Document and print it
    let res = bucket.get("21st_amendment_brewery_cafe-21a_ipa").unwrap();
    println!("{:?}: {:?}", res.id(), res.cas());

    // when cluster goes out of scope, calls "close" on all buckets it owns.
}
