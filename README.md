# SnapAPI
SnapAPI is a blazing fast Rust framework which provides a simple way of creating CRUD APIs by offering HTTP servers and SQL clients.

```rust
 use snapapi::{
   mysql::{
     self,
     MySQL,
   },

   h1::{
     SnapAPI,
     http,
   },
 };

 fn add_user(query: http::QueryParams, cursor: mysql::Cursor) -> http::Response {
   let name = match query.get("name") {
     None => {
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

   cursor.query(format!("INSERT INTO users (name, age) VALUES ({}, {})", name, age));
   http::Response::Plain("Done!")
 }

 fn main() {
   let database = MySQL::new()
     .db("snapapi")
     .username("root")
     .password("my-secret-password");
   let cursor = database.connect(32);

   SnapAPI::new()
     .cursor(cursor)
     .route("/add-user", add_user)
     .run(32);
 }
```

## What it provides
- Extremely low dependency count
- Supports HTTP/1.1, HTTP/2 and HTTP/3 server and client
- Supports MySQL and Postgresql client
- Support for gzip middleware
- Runs on stable Rust
- 100% save code

## What it doesn't provide
- No support for custom middleware
- No support for SSL 
- No support for async
- No support for setting custom headers

## Note
- SnapAPI is only meant for localhost as you can see in what it doesn't provide!
- SnapAPI isn't meant for backend web apps as you can see in what it doesn't provide
- SnapAPIs protocol implementations are mostly lightweight which allows for extreme speeds
- Please only use the http clients provided by SnapAPI for the http servers from SnapAPI:
  > since they are designed to work specifically with SnapAPIs http servers 

  > since they will give you an insane speed-up for SnapAPIs http servers

  > but you can use any http client for a SnapAPI http server
