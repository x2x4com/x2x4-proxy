// use hyper::client::HttpConnector;
use hyper::{Client, Server, header, Uri};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use std::net::IpAddr;

use clap::Parser;

/// Simple http proxy
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// which target you want to proxy
    #[arg(short, long)]
    target: String,

    /// target http or https, default is https
    #[arg(short, long, default_value_t = String::from("https"))]
    schema: String,

    /// ip to listen, default is 127.0.0.1
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    listen: String,

    /// port to listen, default is 3000
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    let socket_addr = SocketAddr::new(args.listen.parse::<IpAddr>().unwrap(), args.port);
    // let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("Start proxy on http://{} to {}://{}", socket_addr, args.schema ,args.target);

    

    let shared = Arc::new(args);
    let make_svc = make_service_fn(move |_conn| {
        let args = shared.clone();
        //println!("start call proxy");
        async move {
            Ok::<_, hyper::Error>(service_fn(move |mut req| {
                // proxy(req, shared)
                // println!("in proxy");

                let http_client = Client::new();
                let https_client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

                let uri_string = format!(
                    "{}://{}{}",
                    args.schema,
                    args.target,
                    req.uri()
                        .path_and_query()
                        .map(|x| x.as_str())
                        .unwrap_or("/")
                );
                let uri: Uri = uri_string.parse().unwrap();
                *req.uri_mut() = uri.clone();
                
                let mut req_header = req.headers().clone();
                
                // req_header.insert(header::HOST, host_port.parse().unwrap());
                req_header.insert(header::HOST, args.target.parse().unwrap());
                req_header.insert("X-Forwarded-Proto", args.schema.parse().unwrap());
                // println!("uri: {}, method: {}, header: {:?}", uri.path(), method, req_header);
                *req.headers_mut() = req_header;
                println!("start to query, {} to {}", req.method(), req.uri());
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
    let server = Server::bind(&socket_addr).serve(make_svc);
    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}