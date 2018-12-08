use rand::thread_rng;
use tiny_http::{Method, Response, Server as HttpServer};

use crate::markov::Markov;

pub struct Server<'a> {
    server: HttpServer,
    markov: &'a Markov<'a>,
}

impl<'a> Server<'a> {
    pub fn new(addr: &str, markov: &'a Markov<'a>) -> Self {
        let server = HttpServer::http(addr).unwrap();
        eprintln!("hosting at http://{}", addr);
        Self { server, markov }
    }
    pub fn start(&mut self) {
        let mut rng = thread_rng();

        for req in self.server.incoming_requests() {
            match (req.method(), req.url()) {
                (&Method::Get, "/markov/next") => {
                    let data = self.markov.generate(&mut rng);
                    let resp = Response::from_string(data);
                    let _ = req.respond(resp);
                }
                (_, _) => {
                    let resp = Response::from_string("404 not found");
                    let resp = resp.with_status_code(404);
                    let _ = req.respond(resp);
                }
            }
        }
    }
}
