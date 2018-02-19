extern crate hyper;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use hyper::server;

// Obtenemos el módulo del servicio
mod lcd_service;

use lcd_service::LcdService;

fn main() {
    info!("Preparing server...");
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = server::Http::new().bind(&addr, || Ok(LcdService)).unwrap();
    info!("Server prepared at address {}", addr);
    server.run().unwrap();
}
