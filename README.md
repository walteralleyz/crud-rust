# CRUD Rust
### *A Crud Project made with Rust, Rocket and Diesel*

> This project aims to be a simple contact keep notes, updatable using API's routes.

#### How to Run this project?
1. Install Rust using [Rustup](https://www.rust-lang.org/tools/install)
2. Run project using inside project directory
~~~
$ cargo build
~~~
3. Install diesel-cli by using cargo install diesel-cli command
4. Type diesel setup --database-url memory.sqlite into console
5. Type diesel migration run --database-url memory.sqlite too
6. Finally
~~~
$ cargo run
~~~
