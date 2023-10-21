# SnapAPI
SnapAPI is a blazingly fast Rust framework which provides a simple way of creating CRUD APIs by offering an HTTP server and SQL clients.

```rust
use snapapi::{
   mysql::{self, MySQL},
   http::{self, SnapAPI}
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

## Features
- Extremely low dependency count
- Implements HTTP/1.1 server and client
- Implements MySQL and Postgresql client
- Runs on stable Rust
- 100% save code

## Usage
> To use snapapi, add this to your Cargo.toml:
```toml
[dependencies]
snapapi = "0.0.1"
```
> Or type this into your terminal:
```text
cargo add snapapi
```

## What it doesn't provide
- No support for custom middleware
- No support for SSL 
- No support for async
- No support for setting custom headers
- No support for compression 

## Disclaimer
- SnapAPI is only meant for localhost as you can see in what it doesn't provide!
- SnapAPI isn't meant for web apps as you can see in what it doesn't provide
- SnapAPIs protocol implementations are mostly lightweight which allows for extreme speeds
- SnapAPI doesn't allow for compression since for lightweight protocol implementations, compression would lead to worse performance 
- Please only use SnapAPIs HTTP client for SnapAPIs HTTP server:
  > Since this HTTP client is designed to work specifically with SnapAPIs HTTP server

  > Since this HTTP client will give you an insane speed-up for SnapAPIs HTTP server

  > But you can use any HTTP client for SnapAPIs HTTP server

## License
This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.
