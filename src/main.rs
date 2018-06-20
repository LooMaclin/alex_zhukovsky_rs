extern crate hyper;
extern crate hyper_tls;

use hyper::client::Client;
use hyper::Body;
use hyper_tls::HttpsConnector;

fn main() {
    let https = HttpsConnector::new(4).unwrap();
    let _client : Client<_, Body> = Client::builder()
        .build(https);
}
