# Readme

### Setup project
Install Rust-lang from
- [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

Run project
```shell
$ cargo run
```

Run mysql on docker
```shell
$ export DATABASE_URL=mysql://root:example@127.0.0.1:3306/heroes
$ docker-compose -f stack.yml up
```
Referencet from [https://hub.docker.com/_/mysql](https://hub.docker.com/_/mysql)

## Solve Isues
- error when install deisel
    -  [https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started)
- connect database with docker
    - `export DATABASE_URL=mysql://root:example@127.0.0.1:3306/heroes`

### Example Reference
[https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8](https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8)
