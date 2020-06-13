use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SimpleTree {
    pub name: Option<String>,
    pub length: Option<f64>,
    pub children: Vec<SimpleTree>,
}
