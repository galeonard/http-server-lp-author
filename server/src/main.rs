use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        (&Method::POST, "/parrot") => {
            let body = hyper::body::to_bytes(req.into_body()).await?;
            let body_str = std::str::from_utf8(&body).unwrap();
            let mut new_body = String::from("You said: ");
            if body_str.is_empty() {
                new_body.push_str("nothing");
            } else {
                new_body.push_str(body_str);
            }
            let body = format!("{}{}", new_body, "\n");
            Ok(Response::new(Body::from(body)))
        }

        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(handle_request)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
