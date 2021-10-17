![Rust](Rust_programming_language_black_logo.svg "Rust")

## Intro

Simple tool for exporting data from a Shopify store to Postgresql using the Shopify GraphQL API.

## Setup

You need to create a private Shopify GraphQL app. This tools in this repo use the basic AUTH schema associated with private Shopify apps and not the more recent OAuth implementation. Adding support for OAuth will be added later.

Once you have a GraphQL app set up, you can run the export scripts by setting the appropriate env vars.
