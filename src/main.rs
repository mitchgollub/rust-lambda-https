mod client;

use client::RustLambdaHttpClient;
// use lambda::{handler_fn, Context};
use lambda_runtime::{handler_fn, Context};
use serde_derive::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = RustLambdaHttpClient::new();
    let client_ref = &client;
    lambda_runtime::run(handler_fn(
        move |event: CustomEvent, _ctx: Context| async move {
            dbg!(&event);
            let url = format!("{host}{path}", host = event.host, path = event.path);
            let body = client_ref
                .clone()
                .send(event.http_verb, url, event.post_data)
                .await?;

            Ok::<CustomOutput, Error>(CustomOutput {
                message: format!("Body: {}", body),
            })
        },
    ))
    .await?;

    Ok(())
}
