use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT; // it shows where the requests are coming from

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);

    // create a client for the request
    let client = reqwest::Client::new();
    let response = client
    .get(&request_url)
    .header(USER_AGENT, "rust web-api-client demo")
    .send()
    .await?;

    // vector with multiple structs
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users); // print all the users
    Ok(())
}