extern crate hyper;
extern crate futures;
extern crate log;
extern crate serde;
extern crate serde_json;

mod implementation;

use hyper::server::{Request, Response, Service};
use hyper::Method;
use hyper::StatusCode;
use futures::{Stream, Future};

use std::str;

pub struct LcdService;

// Esta clase contendrá la respuesta del JSON transformada
#[derive(Deserialize, Debug)]
struct RequestBody<'a> {
    pub t: &'a str,
    pub n: u64
}
// Desactivamos este warning, ya que indica que un valor no está siendo leido, pero indirectamente es necesario crearlo.
#[warn(unused_assignments)]
impl Service for LcdService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _request: Request) -> Self::Future {
        if _request.path() == "/numbers" && _request.method() == &Method::Post {
            return Box::new(_request.body().fold(
                    Vec::new(), |mut acc, chunk| {
                        acc.extend_from_slice(chunk.as_ref());
                        return Ok::<_, Self::Error>(acc);
                    }
                ).and_then(move |byte_res| {
                    let mut response = Response::new();
                    response.set_status(StatusCode::Ok);
                    let mut response_text = String::new();
                    let text_res = str::from_utf8(&byte_res);
                    match text_res {
                        Ok(text) => {
                            let res = serde_json::from_str::<RequestBody>(text);
                            match res {
                                Ok(response_obj) => {
                                    // Ejecutamos el método que devuelve el texto
                                    let res = implementation::render_lcd(response_obj.t ,response_obj.n);
                                    match res {
                                        Ok(lcd_response) => {
                                            response_text = lcd_response;
                                        }
                                        Err(_) => {
                                            response_text = String::from("Error generating LCD display!");
                                            response.set_status(StatusCode::InternalServerError);
                                        }
                                    }
                                },
                                Err(_) => {
                                    response_text = String::from("Error parsing JSON!");
                                    response.set_status(StatusCode::BadRequest);
                                }
                            }
                        },
                        Err(_) => {
                            response_text = String::from("Error parsing body text!");
                            response.set_status(StatusCode::BadRequest);
                        }
                    }
                    response = response.with_body(response_text);

                    return Ok(response);
                }));
        } else {
            return Box::new(futures::future::ok({
                let mut response = Response::new();
                response.set_status(StatusCode::NotFound);
                response
            }));
        }
    }
}