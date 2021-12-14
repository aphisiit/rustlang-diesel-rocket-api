# Readme

## Setup project
Install Rust-lang from
- [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

Run mysql on docker
```shell
$ export DATABASE_URL=mysql://root:example@127.0.0.1:3306/heroes
$ docker-compose -f stack.yml up
```

Install library for MySQL via homebrew
```shell
$ brew install mysql-client mysql
```
If not have that, Install from [https://brew.sh/](https://brew.sh/)

Export MySql Library
```shell
$ echo 'export LDFLAGS="-L/usr/local/opt/mysql-client/lib"' >> ~/.zshrc
$ echo 'export CPPFLAGS="-I/usr/local/opt/mysql-client/include"' >> ~/.zshrc
$ echo 'export PATH=/usr/local/opt/mysql-client/bin:$PATH' >> ~/.zshrc
$ source ~/.zshrc
```

Install diesel cli
```shell
$ cargo install diesel_cli --no-default-features --features postgres
```

Migration database
```shell
$ diesel setup
$ diesel migration run
```
More information [http://diesel.rs/guides/getting-started.html](http://diesel.rs/guides/getting-started.html)

Run project
```shell
$ cargo run
```

Reference from [https://hub.docker.com/_/mysql](https://hub.docker.com/_/mysql)

### Solve Isues
- error when install deisel
    -  [https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started)
- connect database with docker
    - `export DATABASE_URL=mysql://root:example@127.0.0.1:3306/heroes`

## Example Reference
[https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8](https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8)
