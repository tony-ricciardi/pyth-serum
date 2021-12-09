mod client;
mod oracle;

use client::Client;

fn main() {
  let client = Client::from_args();
  println!("{:?}", client);
}
