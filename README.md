# SnapAPI
SnapAPI is a blazingly fast Rust framework which provides a simple way of creating CRUD APIs by offering HTTP servers and SQL clients.

```rust
use snapapi::{
   mysql::{self, MySQL}, 
   h1::{SnapAPI, http}
};

fn add_user(query: http::QueryParams, cursor: mysql::Cursor) -> http::Response {
  // Get the name and age from the query-params 
  let name = match query.get("name") {
    None => {
      // Return a detailed 403 if the name is not provided
      return http::Response::from_status_code(403, "please provide a name");
    },

    Some(name) => name,
  };

  let age = match query.get("age") {
    None => {
      return http::Response::from_status_code(403, "please provide an age");
    },

    Some(age) => age,
  };
   
  // Send the query to the MySQL database
  cursor.query(format!("INSERT INTO users (name, age) VALUES ({}, {})", name, age));
   
  // Send a response to the http client 
  http::Response::Plain("Done!")
}

fn main() {
  let database = MySQL::new() // Create a new MySQL instance
    .db("snapapi") // Specify the name of the database
    .username("root") // Specify the username 
    .password("my-secret-password"); // Specify the users password
  let cursor = database.connect(32); // Specify how many threads the cursor should use
   
  SnapAPI::new() // Create a new SnapAPI (HTTP server) instance
    .cursor(cursor) // Specify the cursor
    .route("/add-user", add_user) // Add a route
    .run(32); // Specify how many threads the cursor should use 
}
```

## What it provides
- Extremely low dependency count
- Supports HTTP/1.1, HTTP/2 and HTTP/3 server and client
- Supports MySQL and Postgresql client
- Runs on stable Rust
- 100% save code

## Usage
> To use snapapi, add this to your Cargo.toml:
```toml
[dependencies]
snapapi = "0.0.1"
```
> or type this into your terminal:
```text
cargo add snapapi
```

## What it doesn't provide
- No support for custom middleware
- No support for SSL 
- No support for async
- No support for setting custom headers
- No support for compression 

## Note
- SnapAPI is only meant for localhost as you can see in what it doesn't provide!
- SnapAPI isn't meant for backend web apps as you can see in what it doesn't provide
- SnapAPIs protocol implementations are mostly lightweight which allows for extreme speeds
- SnapAPI doesn't allow for compression since for crud apis, compression would lead to worse performance 
- Please only use the HTTP clients provided by SnapAPI for the HTTP servers from SnapAPI:
  > since they are designed to work specifically with SnapAPIs HTTP servers 

  > since they will give you an insane speed-up for SnapAPIs HTTP servers

  > but you can use any http client for a SnapAPI HTTP server
