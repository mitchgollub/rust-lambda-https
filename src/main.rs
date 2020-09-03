use hyper::{Body, Client, Method, Request};
use lambda::{lambda, Context};
use serde_derive::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    host: String,
    path: String,
    http_verb: String,
    #[serde(default)]
    post_data: String,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

#[lambda]
#[tokio::main]
async fn main(event: CustomEvent, _: Context) -> Result<CustomOutput, Error> {
    let https = hyper_rustls::HttpsConnector::new();

    let client: Client<_, hyper::Body> = Client::builder().build(https);

    let req = Request::builder()
        .method(Method::from_bytes(event.http_verb.as_bytes()).unwrap())
        .uri(format!(
            "{host}{path}",
            host = event.host,
            path = event.path
        ))
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:79.0) Gecko/20100101 Firefox/79.0",
        )
        .body(Body::from(event.post_data))
        .unwrap();

    let res = client.request(req).await?;

    let buf = hyper::body::to_bytes(res).await?;
    Ok(CustomOutput {
        message: format!("Body: {:?}", buf),
    })
}
