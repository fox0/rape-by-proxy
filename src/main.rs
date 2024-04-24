use std::io::{stdout, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::RootCertStore;
use rustls::craft::{CHROME_108, CHROME_108_EXT, CHROME_CIPHER, CraftPadding, FingerprintBuilder};
use rustls::craft::{Fingerprint, CraftExtension};
use rustls::craft::CraftExtension::FakeApplicationSettings;
use rustls::internal::msgs::enums::ExtensionType;
use rustls::internal::msgs::handshake::ClientExtension;


fn main() {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let fingerprint: &Fingerprint = &CHROME_108.main;
    // let fingerprint = &CHROME_108.test_alpn_http1;

    // let aaa = CHROME_108_EXT.clone();
    // let bbb = CHROME_CIPHER.clone();
    // 
    // let fingerprint = Fingerprint {
    //     extensions: &[],
    //     shuffle_extensions: false,
    //     cipher: &[],
    // };

    // let aaa = CraftExtension::Padding;
    // let aa = ClientExtension::CraftPadding(CraftPadding {
    //     psk_len: 16000
    // });

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
        .with_fingerprint(fingerprint.builder());

    // Allow using SSLKEYLOGFILE.
    // config.key_log = Arc::new(rustls::KeyLogFile::new());

    let server_name = "google.com".try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
    let mut sock = TcpStream::connect("derpicdn.net:443").unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    tls.write_all(
        concat!(
        "GET /img/2024/4/23/3349451/thumb_small.png HTTP/1.1\r\n",
        "Host: derpicdn.net\n",
        "Connection: close\r\n",
        "Accept-Encoding: identity\r\n",
        "\r\n"
        )
            .as_bytes(),
    )
        .unwrap();
    // 

    let ciphersuite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "Current ciphersuite: {:?}",
        ciphersuite.suite()
    ).unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    stdout().write_all(&plaintext).unwrap();
}


// // https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs
//
// use std::net::SocketAddr;
//
// use bytes::Bytes;
// use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
// use hyper::client::conn::http1::Builder;
// use hyper::server::conn::http1;
// use hyper::service::service_fn;
// use hyper::upgrade::Upgraded;
// use hyper::{Method, Request, Response};
//
// use tokio::net::{TcpListener, TcpStream};
//
// // #[path = "../benches/support/mod.rs"]
// // mod support;
// // use support::TokioIo;
// mod tokiort;
// use tokiort::TokioIo;
//
// // To try this example:
// // 1. cargo run --example http_proxy
// // 2. config http_proxy in command line
// //    $ export http_proxy=http://127.0.0.1:8100
// //    $ export https_proxy=http://127.0.0.1:8100
// // 3. send requests
// //    $ curl -i https://www.some_domain.com/
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8100));
//
//     let listener = TcpListener::bind(addr).await?;
//     println!("Listening on http://{}", addr);
//
//     loop {
//         let (stream, _) = listener.accept().await?;
//         let io = TokioIo::new(stream);
//
//         tokio::task::spawn(async move {
//             if let Err(err) = http1::Builder::new()
//                 .preserve_header_case(true)
//                 .title_case_headers(true)
//                 .serve_connection(io, service_fn(proxy))
//                 .with_upgrades()
//                 .await
//             {
//                 println!("Failed to serve connection: {:?}", err);
//             }
//         });
//     }
// }
//
// async fn proxy(
//     req: Request<hyper::body::Incoming>,
// ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
//     println!("req: {:?}", req);
//
//     if Method::CONNECT == req.method() {
//         // Received an HTTP request like:
//         // ```
//         // CONNECT www.domain.com:443 HTTP/1.1
//         // Host: www.domain.com:443
//         // Proxy-Connection: Keep-Alive
//         // ```
//         //
//         // When HTTP method is CONNECT we should return an empty body
//         // then we can eventually upgrade the connection and talk a new protocol.
//         //
//         // Note: only after client received an empty body with STATUS_OK can the
//         // connection be upgraded, so we can't return a response inside
//         // `on_upgrade` future.
//         if let Some(addr) = host_addr(req.uri()) {
//             tokio::task::spawn(async move {
//                 match hyper::upgrade::on(req).await {
//                     Ok(upgraded) => {
//                         if let Err(e) = tunnel(upgraded, addr).await {
//                             eprintln!("server io error: {}", e);
//                         };
//                     }
//                     Err(e) => eprintln!("upgrade error: {}", e),
//                 }
//             });
//
//             Ok(Response::new(empty()))
//         } else {
//             eprintln!("CONNECT host is not socket addr: {:?}", req.uri());
//             let mut resp = Response::new(full("CONNECT must be to a socket address"));
//             *resp.status_mut() = http::StatusCode::BAD_REQUEST;
//
//             Ok(resp)
//         }
//     } else {
//         let host = req.uri().host().expect("uri has no host");
//         let port = req.uri().port_u16().unwrap_or(80);
//
//         let stream = TcpStream::connect((host, port)).await.unwrap();
//         let io = TokioIo::new(stream);
//
//         let (mut sender, conn) = Builder::new()
//             .preserve_header_case(true)
//             .title_case_headers(true)
//             .handshake(io)
//             .await?;
//         tokio::task::spawn(async move {
//             if let Err(err) = conn.await {
//                 println!("Connection failed: {:?}", err);
//             }
//         });
//
//         let resp = sender.send_request(req).await?;
//         Ok(resp.map(|b| b.boxed()))
//     }
// }
//
// fn host_addr(uri: &http::Uri) -> Option<String> {
//     uri.authority().and_then(|auth| Some(auth.to_string()))
// }
//
// fn empty() -> BoxBody<Bytes, hyper::Error> {
//     Empty::<Bytes>::new()
//         .map_err(|never| match never {})
//         .boxed()
// }
//
// fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
//     Full::new(chunk.into())
//         .map_err(|never| match never {})
//         .boxed()
// }
//
// // Create a TCP connection to host:port, build a tunnel between the connection and
// // the upgraded connection
// async fn tunnel(upgraded: Upgraded, addr: String) -> std::io::Result<()> {
//     // Connect to remote server
//     let mut server = TcpStream::connect(addr).await?;
//     let mut upgraded = TokioIo::new(upgraded);
//
//     // Proxying data
//     let (from_client, from_server) =
//         tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;
//
//     // Print message when done
//     println!(
//         "client wrote {} bytes and received {} bytes",
//         from_client, from_server
//     );
//
//     Ok(())
// }