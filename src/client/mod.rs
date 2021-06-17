use crate::Error;
use surf::http::Method;
use surf::Client;
use surf::Url;

#[derive(Clone)]
pub struct RustLambdaHttpClient {
    client: Client,
}

impl RustLambdaHttpClient {
    pub fn new() -> Self {
        RustLambdaHttpClient {
            client: surf::Client::new(),
        }
    }

    pub async fn send(
        self,
        http_verb: String,
        url: String,
        post_data: String,
    ) -> Result<String, Error> {
        let mut res = self.client
            .send(
                surf::RequestBuilder::new(parse_http_method(&http_verb), Url::parse(&url)?)
                    .body(post_data)
                    .header(
                        "User-Agent",
                        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:79.0) Gecko/20100101 Firefox/79.0",
                    )
                    .build(),
            )
            .await?;
        let body = res.body_string().await?;
        dbg!(&body);
        Ok(body)
    }
}

fn parse_http_method(input_method: &str) -> Method {
    match input_method {
        "GET" => Method::Get,
        "POST" => Method::Post,
        _ => panic!("No matching HTTP method for {}", input_method),
    }
}
