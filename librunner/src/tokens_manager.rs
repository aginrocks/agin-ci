use std::collections::HashMap;

use aginci_core::workflow::Job;
use rand::{Rng, distr::Alphanumeric};
use uuid::Uuid;

#[derive(Clone)]
pub struct JobRun {
    pub id: Uuid,
    pub job: Job,
}

pub struct TokensManager {
    pub tokens: HashMap<String, JobRun>,
}

impl TokensManager {
    pub fn new() -> Self {
        TokensManager {
            tokens: HashMap::new(),
        }
    }

    pub fn generate_run_token(&mut self, run: JobRun) -> String {
        // let rng = rand::rngs::ThreadRng::default();

        // let token: String = rng
        //     .sample_iter(&Alphanumeric)
        //     .take(48)
        //     .map(char::from)
        //     .collect();

        let token =
            "aginci_librunner_token_Yz89QVV3h7VGlQzjsOYUGGIxFgmpNoThSxo9sTs1MyDeSJxK".to_string(); // format!("aginci_librunner_token_{token}");

        self.tokens.insert(token.clone(), run);

        token
    }
}

impl Default for TokensManager {
    fn default() -> Self {
        Self::new()
    }
}
