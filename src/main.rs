use lambda_http::{service_fn, Error, IntoResponse, Request, Response};
use rand::{thread_rng, seq::SliceRandom};

const MOUNTAIN_PICS_QUERY: &str = "
SELECT ?item ?pic
WHERE
{
?item wdt:P31 wd:Q8502 .
?item wdt:P18 ?pic
SERVICE wikibase:label { bd:serviceParam wikibase:language \"[AUTO_LANGUAGE],en\" }
}
";

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(what_a_peak)).await?;

    Ok(())
}

async fn what_a_peak(_request: Request) -> Result<impl IntoResponse, Error> {
    let pics = find_mountain_pics().await?;
    let mut rng = thread_rng();
    let peak = pics.choose(&mut rng).expect("The list of peaks should never be empty");
    Ok(Response::builder()
        .status(302)
        .header("Location", peak)
        .body("".to_string())?)
}

async fn find_mountain_pics() -> Result<Vec<String>, Error> {
    let api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").await?; // Will determine the SPARQL API URL via site info data
    let res = api.sparql_query(MOUNTAIN_PICS_QUERY).await?;

    Ok(res
        .as_object()
        .and_then(|root| {
            Some(
                root.get("results")?
                    .get("bindings")?
                    .as_array()?
                    .iter()
                    .flat_map(|binding| {
                        Some(binding.get("pic")?.get("value")?.as_str()?.to_owned())
                    })
                    .collect(),
            )
        })
        .unwrap_or_default())
}

#[tokio::test]
async fn test_what_a_peak() {
    let request = Request::default();
    let response = what_a_peak(request).await.unwrap().into_response().await;
    assert_eq!(
        response.body(),
        &lambda_http::Body::Text("".to_string())
    );

    assert!(response.headers().get("Location").is_some());
}
