# rust-lambda-https

This repository illustrates a way to use Rust with AWS Lambda.  It uses rustls to make an HTTPS request to a TLS server that is designated in the event.  More details in this blog post: <https://github.com/mitchgollub/rust-lambda-https>.

## Develop

Use the `run.sh` script at the root of the directory to run the lambda using the SAM CLI.  The `x86_64-unknown-linux-musl` Rust target will need to be installed and set up.  Once that's complete, cargo will package your app for use in a Lambda.

## Deploy

You should just be able to run `sam deploy` to get this to deploy up to your AWS instance.
