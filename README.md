# What a Peak

## Prerequsites

* [Rust](https://www.rust-lang.org/)
* [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
* [Cargo Lambda](https://github.com/awslabs/aws-lambda-rust-runtime#getting-started)

## Deploy

### First time

```bash
make build
sam deploy --guided
```

The first command will build the source of your application. The second command will package and deploy your application to AWS, with a series of prompts.

You can find your API Gateway Endpoint URL in the output values displayed after deployment.

### Subsequent times

```bash
make build
sam deploy
```

## Fetch, tail, and filter Lambda function logs

```bash
sam logs -n WhatAPeakFunction --stack-name what-a-peak --tail
```

## Tests

```bash
cargo test
```
