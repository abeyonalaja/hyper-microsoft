use hyper::service::{service_fn, service_fn_ok};
use hyper::{Body, Response, Server};
use hyper::rt::Future;

fn main() {

    println!("Hello, world!");
//    let addr = ([127,0,0,1], 8080).into();
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);

//    let server = server.map_err(drop);


    let server = builder.serve(|| {
        service_fn_ok(|_| {
            Response::new(Body::from("Almost microservice..."))
        })
    });

    let server = server.map_err(drop);
    hyper::rt::run(server);
}
