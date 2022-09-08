## Intro

Simple tool for exporting data from a Shopify store to Postgresql using the Shopify GraphQL API.

## Setup

You need to create a private Shopify GraphQL app. The tools in this repo use the basic AUTH schema associated with private Shopify apps and not the more recent OAuth implementation. Support for OAuth can be added later.

Once you have a GraphQL app set up, you can run the export scripts by setting the appropriate env vars.
