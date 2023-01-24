use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "testuser".to_string();
    let password: Option<String> = None;
    let res = client
    .get("http://httpbin.org/get")
    .basic_auth(user, password)
    .send();

    println!("{:?}", res);
    Ok(())
  }
