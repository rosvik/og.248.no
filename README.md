# og.248.no

A simple Rust-based API that pulls [Open Graph](https://ogp.me/) data from a given URL. It works by fetching the HTML content of the URL and returning the values of any Open Graph meta tags. There is a built in cache, which makes it blazingly fast to fetch the same URL multiple times.

A hosted version is available at [https://og.248.no](https://og.248.no).

## How to Use

### Setup

```bash
cargo run
```

The API will start running on `http://127.0.0.1:2340`.

### Usage

Fetch data by making an API call on this format:

`GET /api?url=<URL>`

The response will be a JSON formatted array with pairs of `property` and `content`.

```ts
Array<{
  property: string;
  content: string;
}>
```

## Example

Using the URL for this GitHub repository as an example:

`GET` https://og.248.no/api?url=https://github.com/rosvik/og.248.no

```json
[
  {
    "property": "twitter:image",
    "content": "https://opengraph.githubassets.com/3237a9a882e420d99c3e74a10d9386e5bdce72efe8f025e37e0317833a47629d/rosvik/og.248.no"
  },
  {
    "property": "twitter:site",
    "content": "@github"
  },
  {
    "property": "twitter:card",
    "content": "summary_large_image"
  },
  {
    "property": "twitter:title",
    "content": "GitHub - rosvik/og.248.no: Get Open Graph data from a given URL"
  },
  {
    "property": "twitter:description",
    "content": "Get Open Graph data from a given URL. Contribute to rosvik/og.248.no development by creating an account on GitHub."
  },
  {
    "property": "og:image",
    "content": "https://opengraph.githubassets.com/3237a9a882e420d99c3e74a10d9386e5bdce72efe8f025e37e0317833a47629d/rosvik/og.248.no"
  },
  {
    "property": "og:image:alt",
    "content": "Get Open Graph data from a given URL. Contribute to rosvik/og.248.no development by creating an account on GitHub."
  },
  {
    "property": "og:image:width",
    "content": "1200"
  },
  {
    "property": "og:image:height",
    "content": "600"
  },
  {
    "property": "og:site_name",
    "content": "GitHub"
  },
  {
    "property": "og:type",
    "content": "object"
  },
  {
    "property": "og:title",
    "content": "GitHub - rosvik/og.248.no: Get Open Graph data from a given URL"
  },
  {
    "property": "og:url",
    "content": "https://github.com/rosvik/og.248.no"
  },
  {
    "property": "og:description",
    "content": "Get Open Graph data from a given URL. Contribute to rosvik/og.248.no development by creating an account on GitHub."
  }
]
```

<div align="right"><img src="https://github-production-user-asset-6210df.s3.amazonaws.com/1774972/269361517-d0d8e30e-4a25-4ba2-b926-2a42da1156f8.svg" width="32" alt="248"></div>
