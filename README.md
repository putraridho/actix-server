# Actix-Server

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/Rust-1.72+-orange.svg)](https://www.rust-lang.org/)
[![Actix Version](https://img.shields.io/badge/Actix-4.4+-green.svg)](https://actix.rs/)

A basic Actix web server for experimenting and learning.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Features

- `User` CRUD
- Share mutable state

## Getting Started

To run the server, make sure you have Rust and Cargo installed.

### Prerequisites

- [Rust](https://www.rust-lang.org/) (1.55 or higher)
- [Cargo](https://crates.io/)

### Installation

1. Clone this repository.

```bash
git clone https://github.com/putraridho/actix-server.git
```

2. Change directory to the project folder.

```bash
cd actix-server
```

3.Build the project.

```bash
cargo build
```

4. Run the server.

```bash
cargo run
```

## Usage

You can create custom routes and endpoints to serve your specific application needs. Here's an example of how to define a basic route:

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer};

#[get("/hello")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## License
This project is licensed under the MIT License - see the LICENSE file for details.

```
This minimal README provides a quick overview of your simple Actix Server playground project.
You can expand or customize it as needed in the future.
```
