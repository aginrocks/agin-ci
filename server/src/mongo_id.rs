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

pub mod vec_oid_to_vec_string {
    use bson::oid::ObjectId;
    use mongodb::bson;
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(id: &Vec<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            id.iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .serialize(serializer)

            // serializer.serialize_newtype_struct(&id.iter().map(|id| id.to_string()).collect::<Vec<String>>())
        } else {
            id.serialize(serializer)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<ObjectId>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)] // Tries to deserialize into one of the variants
        enum VecStringOrVecObjectId {
            VecString(Vec<String>),
            VecObjectId(Vec<ObjectId>),
        }

        let value = VecStringOrVecObjectId::deserialize(deserializer)?;
        match value {
            VecStringOrVecObjectId::VecObjectId(ids) => Ok(ids), // Already Vec<ObjectId>
            VecStringOrVecObjectId::VecString(strings) => {
                // Convert Vec<String> to Vec<ObjectId>
                strings
                    .into_iter()
                    .map(|s| ObjectId::parse_str(&s).map_err(serde::de::Error::custom))
                    .collect::<Result<Vec<ObjectId>, D::Error>>()
            }
        }
    }
}
