use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
#[tokio::main]
fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_sve = make_service_fn(|_conn|{
        await {
            Ok::<_,Infallible>(service_fn(handle_request))
        }
    });

    let server = Server::bind(&addr).serve(make_sve);
    println!("Server running at http::{}",addr)

    if let Err(e) ==server.await{
        eprintln!("server error:{}",e);
    }
}
