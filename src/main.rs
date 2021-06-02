use lambda::{lambda, Context};
use serde_derive::{Deserialize, Serialize};
use surf::http::Method;
use surf::Url;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Deserialize)]
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
    dbg!(&event);
    let url = Url::parse(&format!(
        "{host}{path}",
        host = event.host,
        path = event.path
    ))?;
    let mut res = surf::Client::new()
        .send(
            surf::RequestBuilder::new(parse_http_method(&event.http_verb), url)
                .body(event.post_data)
                .header(
                    "User-Agent",
                    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:79.0) Gecko/20100101 Firefox/79.0",
                )
                .build(),
        )
        .await?;
    let body = res.body_string().await?;
    dbg!(&body);

    Ok(CustomOutput {
        message: format!("Body: {}", body),
    })
}

fn parse_http_method(input_method: &str) -> Method {
    match input_method {
        "GET" => Method::Get,
        "POST" => Method::Post,
        _ => panic!("No matching HTTP method for {}", input_method),
    }
}
