# rust-lambda-https

This repository illustrates a way to use Rust with AWS Lambda.  The Lambda uses bootstrap code provided by AWS that allows for Rust code to run in AWS Lambda.  The example in this repo uses [surf](https://docs.rs/surf/2.2.0/surf/) to make an HTTPS request that is described in the event.  More details in this blog post: <https://mitchgollub.com/rust-and-aws-lambda/>.

## Installations

Install AWS SAM as per https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html

Install Docker using your OS distribution's package repository or using instructions from https://docs.docker.com/engine/install/

Install Rust (including rustup and cargo) using instructions at https://www.rust-lang.org/tools/install

To build the project for the `x86_64-unknown-linux-musl` target environment that it will deployed into, you will also need to install:
* musl-tools (on Linux: `sudo apt install musl-tools`)
* the pre-built `std` Rust package for musl. Use `rustup target add x86_64-unknown-linux-musl`

## Develop

Use the `run.sh` script at the root of the directory to build and run (locally) the lambda using the SAM CLI. This will also package your lambda for deployment onto AWS.

## Deploy

Run `sam deploy --guided` to deploy the lambda into your AWS account (as shown in [Deploy your application to the AWS Cloud](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started-hello-world.html#serverless-getting-started-hello-world-deploy)).  Use the stack name `rust-lambda-https`.

Test the Lambda by going to the Test tab for your function in the AWS console. create a test event using one on the examples within `events`.

Clean up your resources by running `aws cloudformation delete-stack --stack-name rust-lambda-https --region <region>` (as shown in [Clean up](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started-hello-world.html#serverless-getting-started-hello-world-cleanup)).

SAM create an S3 bucket during deployment that isn't removed. You probably want to keep this if you intend to use SAM again in the future, but you may wish to delete experimental deployments within it.

