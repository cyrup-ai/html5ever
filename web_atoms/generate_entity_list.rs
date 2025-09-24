use std::collections::BTreeMap;
mod entities;

fn main() {
    let mut entities_map: BTreeMap<String, (u32, u32)> = entities::NAMED_ENTITIES
        .iter()
        .map(|(name, cp1, cp2)| (name[1..].to_string(), (*cp1, *cp2)))
        .collect();

    let mut prefixes_to_add = Vec::new();
    for key in entities_map.keys() {
        for n in 1..key.len() {
            prefixes_to_add.push(key[..n].to_string());
        }
    }

    for prefix in prefixes_to_add {
        entities_map.entry(prefix).or_insert((0, 0));
    }
    entities_map.insert("".to_string(), (0, 0));

    println!("pub const ALL_ENTITIES: &[(&'static str, (u32, u32))] = &[");
    for (key, value) in &entities_map {
        println!("    ({:?}, ({}, {})),", key, value.0, value.1);
    }
    println!("];");
}
