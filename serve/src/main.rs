use lambda_http::{service_fn, Error, IntoResponse, Request, Response};
use rand::{seq::IteratorRandom, thread_rng};

const PICS: &str = include_str!("../../data/pics.txt");

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(what_a_peak)).await?;

    Ok(())
}

async fn what_a_peak(_request: Request) -> Result<impl IntoResponse, Error> {
    let mut rng = thread_rng();
    let peak = PICS
        .lines()
        .choose(&mut rng)
        .expect("The list of peaks should never be empty");
    Ok(Response::builder()
        .status(302)
        .header("Location", peak)
        .body("".to_string())?)
}

#[tokio::test]
async fn test_what_a_peak() {
    let request = Request::default();
    let response = what_a_peak(request).await.unwrap().into_response().await;
    assert_eq!(response.body(), &lambda_http::Body::Text("".to_string()));

    assert!(response.headers().get("Location").is_some());
}
