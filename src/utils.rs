pub mod serializable_uuid {
    use uuid::Uuid;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn _serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_string().serialize(serializer)
    }

    pub fn _deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        Uuid::try_parse(val).map_err(D::Error::custom)
    }
}
