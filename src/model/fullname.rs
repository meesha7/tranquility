use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Fullname(String);
