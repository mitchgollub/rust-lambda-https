use lambda::{lambda, Context};
use rustls;
use serde_derive::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use webpki;
use webpki_roots;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    host: String,
    path: String,
    http_verb: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_data: Option<String>,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

#[lambda]
#[tokio::main]
async fn main(event: CustomEvent, _: Context) -> Result<CustomOutput, Error> {
    let mut config = rustls::ClientConfig::new();
    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let dns_name = webpki::DNSNameRef::try_from_ascii_str(event.host.as_str()).unwrap();
    let mut sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut sock = TcpStream::connect(format!("{}:443", event.host)).unwrap();
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);
    let mut http_request: String = String::from("");

    http_request.push_str(&format!(
        "{verb} {path} HTTP/1.1\r\n",
        verb = event.http_verb.as_str(),
        path = event.path.as_str()
    ));
    http_request.push_str(&format!("Host: {}\r\n", event.host.as_str()));

    match event.post_data {
        Some(post_data) => {
            http_request.push_str("Connection: keep-alive\r\n");
            http_request.push_str("Content-Type: application/json\r\n");
            http_request.push_str("User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:79.0) Gecko/20100101 Firefox/79.0\r\n");
            http_request.push_str("Accept: application/json\r\n");
            http_request.push_str("\r\n");
            http_request.push_str(post_data.as_str());
        }
        None => {
            http_request.push_str("Connection: close\r\n");
            http_request.push_str("Accept-Encoding: identity\r\n");
        }
    }
    http_request.push_str("\r\n");

    tls.write(http_request.as_bytes()).unwrap();

    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();

    Ok(CustomOutput {
        message: format!("Body: {}", String::from_utf8(plaintext).unwrap()),
    })
}
