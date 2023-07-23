use bit_set::BitSet;
use serde::{Deserialize, Deserializer, Error, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
struct SerializableVecBool {
    #[serde(
        serialize_with = "serialize_bitset",
        deserialize_with = "deserialize_bitset"
    )]
    inner: Vec<bool>,
}

fn serialize_bitset<S>(bitset: &Vec<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bs: BitSet = bitset
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { Some(i) } else { None })
        .collect();
    bs.serialize(serializer)
}

fn deserialize_bitset<'de, D>(deserializer: D) -> Result<Vec<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let bs: BitSet = Deserialize::deserialize(deserializer)?;
    let mut vec = Vec::with_capacity(bs.len());
    for i in 0..bs.len() {
        vec.push(bs.contains(i));
    }
    Ok(vec)
}
