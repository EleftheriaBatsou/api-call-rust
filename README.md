### Making API calls 🦀

The `main` function is marked with the `#[tokio::main]` attribute, indicating that it is the entry point of the program and will use Tokio's runtime for asynchronous execution. A URL for the API request is constructed using string formatting. This URL is for the GitHub API endpoint that retrieves the list of users who have starred the "rust-cookbook" repository in the "rust-lang-nursery" organization. 
The constructed URL is printed to the console for debugging purposes.

A new `reqwest::Client` instance is created to make the HTTP request. 

The client sends a GET request to the constructed URL with the `USER_AGENT` header set to "rust web-api-client demo". 

The `send()` method sends the request asynchronously, and `await` is used to asynchronously wait for the response. 

Any errors encountered during the request are handled with `?`.

The response body is deserialized into a vector of `User` structs using the `json()` method provided by `reqwest`. 

The `await` keyword is used to asynchronously wait for the completion of JSON deserialization. 

Any errors encountered during deserialization are handled with `?`.

The vector of `User` structs is printed to the console for debugging purposes.

**Overall, this code demonstrates how to:**

✓ make an HTTP GET request to an API endpoint using the `reqwest` crate

✓ handle the response asynchronously using `async` and `await`

✓ deserialize the JSON response into Rust structs using `serde`

✓ additionally, it sets the `User-Agent` header in the request to identify the client making the request
