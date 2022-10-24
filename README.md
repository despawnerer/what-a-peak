# What a Peak

Huh?

## Prerequsites

* [Rust](https://www.rust-lang.org/)

## Structure

### [`data`](data)

- `pics.txt` containing the list of URLs of pictures to be served

### [`generate`](generate)

Binary crate that regenerates `data/pics.txt` from wikidata.

### [`serve`](serve)

Lambda that redirects to a random URL from `data/pics.txt`.

## Deployment

Refer to [`serve` module's README](serve/README.md)
