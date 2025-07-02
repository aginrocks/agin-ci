pub mod object_id_as_string {
    use bson::oid::ObjectId;
    use mongodb::bson;
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            match id {
                Some(id) => serializer.serialize_some(&id.to_string()),
                None => serializer.serialize_none(),
            }
        } else {
            // Serialize as native ObjectId for BSON and others
            id.serialize(serializer)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrObjectId {
            String(String),
            ObjectId(ObjectId),
        }

        let opt = Option::<StringOrObjectId>::deserialize(deserializer)?;
        match opt {
            Some(StringOrObjectId::ObjectId(id)) => Ok(Some(id)),
            Some(StringOrObjectId::String(s)) => ObjectId::parse_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

pub mod object_id_as_string_required {
    use bson::oid::ObjectId;
    use mongodb::bson;
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&id.to_string())
        } else {
            id.serialize(serializer)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrObjectId {
            String(String),
            ObjectId(ObjectId),
        }

        let value = StringOrObjectId::deserialize(deserializer)?;
        match value {
            StringOrObjectId::ObjectId(id) => Ok(id),
            StringOrObjectId::String(s) => {
                ObjectId::parse_str(&s).map_err(serde::de::Error::custom)
            }
        }
    }
}
