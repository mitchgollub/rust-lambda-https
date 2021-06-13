# rust-lambda-https

This repository illustrates a way to use Rust with AWS Lambda.  The Lambda uses bootstrap code provided by AWS that allows for Rust code to run in AWS Lambda.  The example in this repo uses [surf](https://docs.rs/surf/2.2.0/surf/) to make an HTTPS request that is described in the event.  More details in this blog post: <https://mitchgollub.com/rust-and-aws-lambda/>.

## Develop

Use the `run.sh` script at the root of the directory to run the lambda using the SAM CLI.  The `x86_64-unknown-linux-musl` Rust target will need to be installed and set up.  Once that's complete, cargo will package your app for use in a Lambda.

## Deploy

Run `sam deploy` to get this to deploy up to your AWS instance.
