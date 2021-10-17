use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

// See https://stackoverflow.com/questions/51550167/how-to-manually-return-a-result-boxdyn-error for more info
#[derive(Debug)]
pub struct GraphQLFetchError();

impl Error for GraphQLFetchError {}

impl Display for GraphQLFetchError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "There was an error fetching from GraphQL")
    }
}
