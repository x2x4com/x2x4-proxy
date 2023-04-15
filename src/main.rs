use hyper::client::HttpConnector;
use hyper::{Client, Server, header, Uri};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use hyper_tls::HttpsConnector;
use std::sync::Arc;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long)]
    host: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("https"))]
    schema: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

/* 
async fn proxy(req: Request<Body>, args: Arc<Args>) -> Result<Response<Body>, hyper::Error> {
    println!("in proxy");
    let client = Client::new();
    let uri = req.uri().clone();
    let method  = req.method().clone();
    let mut req_header = req.headers().clone();
    req_header.insert(header::HOST, args.host.parse().unwrap());
    println!("uri: {}, method: {}, header: {:?}", uri.path(), method, req_header);
    let mut proxied_req = Request::new(req.into_body());
    *proxied_req.method_mut() = method;
    *proxied_req.uri_mut() = format!("{}://{}{}",args.schema ,args.host, uri.path()).parse().unwrap();
    *proxied_req.headers_mut() = req_header;
    println!("start to query, {:?}", proxied_req);
    // let res = client.request(proxied_req).await?;
    let res2 = client.get(Uri::from_static("https://ai.x2x4.net")).await?;
    println!("status code: {}", res2.status());
    println!("finish query, res body: {:?}", res2.body());
    Ok(res2)
}
*/

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("Start proxy on http://{} to {}://{}", addr, args.schema ,args.host);

    

    let shared = Arc::new(args);
    let make_svc = make_service_fn(move |_conn| {
        let args = shared.clone();
        println!("start call proxy");
        async move {
            Ok::<_, hyper::Error>(service_fn(move |mut req| {
                // let args = shared.clone();
                // proxy(req, shared)
                println!("in proxy");

                let http_client = Client::new();
                let https_client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

                //let mut remote_port: u16 = 80;
                //if args.schema == "https" {
                //    remote_port = 443;
                //}

                // let out_addr: SocketAddr = SocketAddr::new(out_ip, remote_port);
                //let out_addr = SocketAddr::from(([35, 223, 50, 38], remote_port));
                //let host_port = format!("{}:{}", args.host, remote_port);

                let uri_string = format!(
                    "{}://{}{}",
                    args.schema,
                    args.host,
                    req.uri()
                        .path_and_query()
                        .map(|x| x.as_str())
                        .unwrap_or("/")
                );
                let uri: Uri = uri_string.parse().unwrap();
                *req.uri_mut() = uri.clone();
                
                let mut req_header = req.headers().clone();
                
                // req_header.insert(header::HOST, host_port.parse().unwrap());
                req_header.insert(header::HOST, args.host.parse().unwrap());
                req_header.insert("X-Forwarded-Proto", args.schema.parse().unwrap());
                // println!("uri: {}, method: {}, header: {:?}", uri.path(), method, req_header);
                *req.headers_mut() = req_header;
                println!("start to query, {:?}", req);
                if args.schema == "https" {
                    https_client.request(req)
                } else {
                    http_client.request(req)
                }
                
                // let res = client.request(proxied_req).await?;
                // println!("status code: {}", res2.status());
                // println!("finish query, res body: {:?}", res2.body());
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}