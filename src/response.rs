use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: HashMap<String, T>,
}

// impl<T> Response<T> {
//     pub fn new() -> Self {
//         Self { results: vec![] }
//     }
// }
