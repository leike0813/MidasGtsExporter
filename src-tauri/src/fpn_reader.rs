use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::error::{ParserError, Result};
use crate::model::{Element, GroupMap, Id, Mesh, Node};

const ELEMENT_3D_TYPES: &[(&str, usize)] = &[("HEXA", 8), ("PRISM", 6), ("PYRAM", 5), ("TETRA", 4)];
const ELEMENT_2D_TYPES: &[(&str, usize)] = &[("RECT", 4), ("TRIA", 3)];
const ELEMENT_1D_TYPES: &[(&str, usize)] = &[("LINE", 2)];
const KEYWORDS: &[&str] = &[
    "HEXA", "PRISM", "PYRAM", "TETRA", "RECT", "TRIA", "LINE", "NODE", "MSET", "MSETE", "MSETN",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct KeywordBlock {
    keyword: String,
    data: Vec<String>,
}

pub fn parse_fpn_file(path: impl AsRef<Path>) -> Result<Mesh> {
    let path = path.as_ref();
    if !path.is_file() {
        return Err(ParserError::InputFileNotFound(path.to_path_buf()));
    }

    let bytes = fs::read(path).map_err(|source| ParserError::ReadFailed {
        path: path.to_path_buf(),
        source,
    })?;
    let content = String::from_utf8_lossy(&bytes);
    parse_fpn_str(&content)
}

pub fn parse_fpn_str(content: &str) -> Result<Mesh> {
    let blocks = collect_keyword_blocks(content);
    build_mesh(&blocks)
}

fn collect_keyword_blocks(content: &str) -> Vec<KeywordBlock> {
    let lines: Vec<&str> = content.lines().collect();
    let mut blocks = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        let Some(keyword) = recognized_keyword(line) else {
            i += 1;
            continue;
        };

        let tokens: Vec<String> = line.split(',').map(ToOwned::to_owned).collect();
        let mut block = KeywordBlock {
            keyword,
            data: tokens.into_iter().skip(1).collect(),
        };

        while i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if !next_line.starts_with(',') {
                break;
            }

            block
                .data
                .extend(next_line.split(',').skip(1).map(ToOwned::to_owned));
            i += 1;
        }

        blocks.push(block);
        i += 1;
    }

    blocks
}

fn recognized_keyword(line: &str) -> Option<String> {
    let keyword = line.split(',').next()?.trim().to_ascii_uppercase();
    KEYWORDS
        .iter()
        .any(|candidate| *candidate == keyword)
        .then_some(keyword)
}

fn build_mesh(blocks: &[KeywordBlock]) -> Result<Mesh> {
    let mut mesh = Mesh::default();
    let mut all_group_names = BTreeMap::new();
    let mut all_element_groups: GroupMap = BTreeMap::new();

    for block in blocks {
        match block.keyword.as_str() {
            "NODE" => {
                let node = parse_node(block)?;
                insert_unique(&mut mesh.nodes, node.id, node, "node")?;
            }
            keyword if element_node_count(keyword, ELEMENT_3D_TYPES).is_some() => {
                let element = parse_element(
                    block,
                    element_node_count(keyword, ELEMENT_3D_TYPES).unwrap(),
                )?;
                insert_unique(&mut mesh.elements_3d, element.id, element, "3d element")?;
            }
            keyword if element_node_count(keyword, ELEMENT_2D_TYPES).is_some() => {
                let element = parse_element(
                    block,
                    element_node_count(keyword, ELEMENT_2D_TYPES).unwrap(),
                )?;
                insert_unique(&mut mesh.elements_2d, element.id, element, "2d element")?;
            }
            keyword if element_node_count(keyword, ELEMENT_1D_TYPES).is_some() => {
                let element = parse_element(
                    block,
                    element_node_count(keyword, ELEMENT_1D_TYPES).unwrap(),
                )?;
                insert_unique(&mut mesh.elements_1d, element.id, element, "1d element")?;
            }
            "MSET" => {
                let (group_id, group_name) = parse_group_name(block)?;
                if group_id > 1 {
                    insert_unique(&mut all_group_names, group_id, group_name, "group name")?;
                }
            }
            "MSETE" => {
                let (group_id, ids) = parse_id_group(block)?;
                if group_id > 1 && !ids.is_empty() {
                    insert_unique(&mut all_element_groups, group_id, ids, "element group")?;
                }
            }
            _ => {}
        }
    }

    classify_element_groups(&mut mesh, &all_element_groups)?;
    mesh.element_groups = mesh
        .element_3d_groups
        .iter()
        .chain(mesh.element_2d_groups.iter())
        .chain(mesh.element_1d_groups.iter())
        .map(|(group_id, ids)| (*group_id, ids.clone()))
        .collect();
    mesh.element_group_names = collect_group_names(&mesh.element_groups, &all_group_names)?;

    for block in blocks.iter().filter(|block| block.keyword == "MSETN") {
        let group_id = parse_group_id(block)?;
        if group_id > 1 && mesh.element_groups.contains_key(&group_id) {
            let (_, ids) = parse_id_group(block)?;
            if !ids.is_empty() {
                insert_unique(&mut mesh.node_groups, group_id, ids, "node group")?;
            }
        }
    }
    mesh.node_group_names = collect_group_names(&mesh.node_groups, &all_group_names)?;

    Ok(mesh)
}

fn element_node_count(keyword: &str, table: &[(&str, usize)]) -> Option<usize> {
    table
        .iter()
        .find_map(|(candidate, node_count)| (*candidate == keyword).then_some(*node_count))
}

fn parse_node(block: &KeywordBlock) -> Result<Node> {
    require_len(block, 4)?;
    Ok(Node::new(
        parse_i32("node id", &block.data[0])?,
        parse_f64("node x", &block.data[1])?,
        parse_f64("node y", &block.data[2])?,
        parse_f64("node z", &block.data[3])?,
    ))
}

fn parse_element(block: &KeywordBlock, node_count: usize) -> Result<Element> {
    require_len(block, 2 + node_count)?;
    let id = parse_i32("element id", &block.data[0])?;
    let node_ids = block
        .data
        .iter()
        .skip(2)
        .take(node_count)
        .enumerate()
        .map(|(index, value)| parse_i32(&format!("element node {}", index + 1), value))
        .collect::<Result<Vec<_>>>()?;
    Ok(Element::new(id, node_ids))
}

fn parse_group_name(block: &KeywordBlock) -> Result<(Id, String)> {
    require_len(block, 2)?;
    Ok((
        parse_i32("group id", &block.data[0])?,
        block.data[1].trim().to_owned(),
    ))
}

fn parse_group_id(block: &KeywordBlock) -> Result<Id> {
    require_len(block, 1)?;
    parse_i32("group id", &block.data[0])
}

fn parse_id_group(block: &KeywordBlock) -> Result<(Id, Vec<Id>)> {
    require_len(block, 8)?;
    let group_id = parse_i32("group id", &block.data[0])?;
    let item_count = parse_usize("group item count", &block.data[1])?;
    require_len(block, 8 + item_count)?;
    let ids = block
        .data
        .iter()
        .skip(8)
        .take(item_count)
        .enumerate()
        .map(|(index, value)| parse_i32(&format!("group item {}", index + 1), value))
        .collect::<Result<Vec<_>>>()?;
    Ok((group_id, ids))
}

fn classify_element_groups(mesh: &mut Mesh, groups: &GroupMap) -> Result<()> {
    for (group_id, ids) in groups {
        let first_id = ids[0];
        if mesh.elements_3d.contains_key(&first_id) {
            mesh.element_3d_groups.insert(*group_id, ids.clone());
        } else if mesh.elements_2d.contains_key(&first_id) {
            mesh.element_2d_groups.insert(*group_id, ids.clone());
        } else if mesh.elements_1d.contains_key(&first_id) {
            mesh.element_1d_groups.insert(*group_id, ids.clone());
        }
    }
    Ok(())
}

fn collect_group_names(
    groups: &GroupMap,
    all_group_names: &BTreeMap<Id, String>,
) -> Result<BTreeMap<Id, String>> {
    groups
        .keys()
        .map(|group_id| {
            all_group_names
                .get(group_id)
                .cloned()
                .map(|name| (*group_id, name))
                .ok_or(ParserError::MissingGroupName {
                    group_id: *group_id,
                })
        })
        .collect()
}

fn insert_unique<T>(map: &mut BTreeMap<Id, T>, id: Id, value: T, kind: &'static str) -> Result<()> {
    if map.insert(id, value).is_some() {
        return Err(ParserError::DuplicateId { kind, id });
    }
    Ok(())
}

fn require_len(block: &KeywordBlock, expected: usize) -> Result<()> {
    if block.data.len() < expected {
        return Err(ParserError::MalformedRecord {
            keyword: block.keyword.clone(),
            detail: format!(
                "expected at least {expected} fields, got {}",
                block.data.len()
            ),
        });
    }
    Ok(())
}

fn parse_i32(field: &str, value: &str) -> Result<Id> {
    value
        .trim()
        .parse::<Id>()
        .map_err(|source| ParserError::MalformedInteger {
            field: field.to_owned(),
            value: value.to_owned(),
            source,
        })
}

fn parse_usize(field: &str, value: &str) -> Result<usize> {
    value
        .trim()
        .parse::<usize>()
        .map_err(|source| ParserError::MalformedInteger {
            field: field.to_owned(),
            value: value.to_owned(),
            source,
        })
}

fn parse_f64(field: &str, value: &str) -> Result<f64> {
    value
        .trim()
        .parse::<f64>()
        .map_err(|source| ParserError::MalformedFloat {
            field: field.to_owned(),
            value: value.to_owned(),
            source,
        })
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::parse_fpn_file;

    fn fixture_path(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("references")
            .join("MidasGtsExporter")
            .join("TestData")
            .join("MidasGts")
            .join(name)
    }

    #[test]
    fn reference_fixtures_parse_successfully() {
        for name in [
            "GtsModel_Test.fpn",
            "GtsModel_Coarse.fpn",
            "GtsModel_Fine.fpn",
        ] {
            let mesh = parse_fpn_file(fixture_path(name)).expect("fixture should parse");
            assert!(!mesh.nodes.is_empty(), "{name} should contain nodes");
            assert!(
                mesh.total_element_count() > 0,
                "{name} should contain elements"
            );
            assert!(
                !mesh.element_groups.contains_key(&1),
                "{name} should ignore default element group"
            );
            assert!(
                !mesh.node_groups.contains_key(&1),
                "{name} should ignore default node group"
            );
            assert_ids_are_sorted(mesh.nodes.keys().copied());
            assert_ids_are_sorted(mesh.elements_3d.keys().copied());
            assert_ids_are_sorted(mesh.elements_2d.keys().copied());
            assert_ids_are_sorted(mesh.elements_1d.keys().copied());
            assert_ids_are_sorted(mesh.element_groups.keys().copied());
            assert_ids_are_sorted(mesh.node_groups.keys().copied());
        }
    }

    #[test]
    fn reference_fixture_continuation_lines_are_merged() {
        let mesh = parse_fpn_file(fixture_path("GtsModel_Test.fpn")).expect("fixture should parse");
        let group = mesh
            .element_groups
            .get(&10)
            .expect("fixture group 10 should exist");
        assert!(
            group.len() > 8,
            "group 10 spans continuation lines and should contain more than one line of ids"
        );
    }

    fn assert_ids_are_sorted(ids: impl IntoIterator<Item = i32>) {
        let ids: Vec<i32> = ids.into_iter().collect();
        assert!(
            ids.windows(2).all(|window| window[0] <= window[1]),
            "ids should be sorted: {ids:?}"
        );
    }
}
