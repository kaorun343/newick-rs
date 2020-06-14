use crate::tree::{FromNewick, ToNewick};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SimpleTree {
    pub name: String,
    pub length: Option<f64>,
    pub children: Vec<SimpleTree>,
}

impl SimpleTree {
    pub fn new(name: String, length: Option<f64>, children: Vec<SimpleTree>) -> Self {
        Self {
            name,
            length,
            children,
        }
    }
}

impl FromNewick for SimpleTree {
    fn leaf(name: String) -> Self {
        Self::new(name, None, Vec::new())
    }

    fn internal(name: String, children: Vec<Self>) -> Self {
        Self::new(name, None, children)
    }

    fn update_length(self, length: Option<f64>) -> Self {
        Self::new(self.name, length, self.children)
    }
}

impl ToNewick for SimpleTree {
    type Child = SimpleTree;

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_children<'a>(&'a self) -> Vec<&'a Self::Child> {
        self.children.iter().collect()
    }

    fn get_length(&self) -> Option<f64> {
        self.length
    }
}
