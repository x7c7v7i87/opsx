use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayloadJson {
    pub name: String,
    pub security_key:String,
    pub git_type:String,
    pub git_url: String,
    pub git_branch:String,
    pub ext_script: String,
}

impl PayloadJson{
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn from_json_vec(json: &str) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_str(json)
    }
}

