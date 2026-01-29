# Minimal TCP Server in Rust

This project is a simple TCP server written in Rust.  
It listens on `127.0.0.1:8080`, accepts incoming connections, reads data sent by the client, and prints it to the terminal.

This program shows how low-level networking works in Rust, before adding HTTP logic.

---

## What the Program Does

- Listens on `127.0.0.1:8080`
- Accepts incoming TCP connections
- Reads up to 1024 bytes from each connection
- Prints the received data to the console
- Does not send any response back to the client