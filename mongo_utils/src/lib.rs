pub use mongo_utils_derive::*;
pub use mongodb::bson::{Document, doc};

pub trait JoinPipelineBuilder {
    fn join_pipeline(local_field: &str, foreign_field: &str) -> Vec<Document>;
}
