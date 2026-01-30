Basic HTTP Server in Rust

This project is a basic HTTP server written in Rust using the standard library.
It listens on 127.0.0.1:8080, reads incoming HTTP requests, and sends plain text responses based on the request method and path.

The goal of this project is to understand how HTTP works at a low level, without using any web frameworks.

What This Server Does

- Listens for TCP connections on 127.0.0.1:8080
- Reads raw HTTP requests from clients
- Separates headers and request body
- Extracts the HTTP method, path, and version
- Handles a small set of routes
- Sends a valid HTTP response back to the client

Supported Routes

GET  /           -> Base route message
POST /           -> POST confirmation message
GET  /health     -> OK
GET  /say_hello  -> Hello
Any  other route -> Not handled message

How to Run

cargo run

How to Test

You can test the server using curl, Postman, or a browser.

Example:
curl http://127.0.0.1:8080/health

Notes

- Responses always return status 200 OK
- One request per connection
- Blocking I/O
- No external libraries used

Purpose

This project is for learning how HTTP works internally and how to build a server from scratch.