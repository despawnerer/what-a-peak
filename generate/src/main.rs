use anyhow::Result;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::PathBuf;

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
async fn main() -> Result<()> {
    println!("Fetching mountain pics...");
    let pics = find_mountain_pics().await?;

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("data");
    path.push("pics.txt");

    println!("Got {}. Writing into {}...", pics.len(), path.to_string_lossy());
    let file = File::create(path)?;
    let mut file = LineWriter::new(file);
    file.write_all(pics.join("\n").as_bytes())?;
    file.flush()?;

    println!("All done! Thank you for flying What a Peak Airlines.");
    Ok(())
}

async fn find_mountain_pics() -> Result<Vec<String>> {
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
