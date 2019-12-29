use hyper::service::{service_fn, service_fn_ok};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use futures::{future, Future};

type UserId = u64;
struct UserData;


const INDEX: &'static str = r#"
 <!doctype html>
 <html>
     <head>
         <title>Rust Microservice</title>
     </head>
     <body>
         <h3>Rust Microservice</h3>
     </body>
 </html>
 "#;

fn microservice_handler(req: Request<Body>) -> impl Future<Item=Response<Body>, Error=Error>
{

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            future::ok(response)
        },
    }
}

fn main() {

    println!("Hello, world!");
//    let addr = ([127,0,0,1], 8080).into();
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);

//    let server = server.map_err(drop);


    let server = builder.serve(|| service_fn(microservice_handler));

    let server = server.map_err(drop);
    hyper::rt::run(server);
}
