use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type Id = i32;
pub type IdMap<T> = BTreeMap<Id, T>;
pub type GroupMap = BTreeMap<Id, Vec<Id>>;
pub type GroupNameMap = BTreeMap<Id, String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: Id,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Node {
    pub fn new(id: Id, x: f64, y: f64, z: f64) -> Self {
        Self { id, x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Element {
    pub id: Id,
    pub node_ids: Vec<Id>,
}

impl Element {
    pub fn new(id: Id, node_ids: Vec<Id>) -> Self {
        Self { id, node_ids }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Mesh {
    pub nodes: IdMap<Node>,
    pub elements_3d: IdMap<Element>,
    pub elements_2d: IdMap<Element>,
    pub elements_1d: IdMap<Element>,
    pub element_groups: GroupMap,
    pub element_3d_groups: GroupMap,
    pub element_2d_groups: GroupMap,
    pub element_1d_groups: GroupMap,
    pub node_groups: GroupMap,
    pub element_group_names: GroupNameMap,
    pub node_group_names: GroupNameMap,
}

impl Mesh {
    pub fn total_element_count(&self) -> usize {
        self.elements_3d.len() + self.elements_2d.len() + self.elements_1d.len()
    }
}
