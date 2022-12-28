/*
 * G4 API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct G4UserImportMessage {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "host")]
    pub host: String,
}

impl G4UserImportMessage {
    pub fn new(username: String, id: i32, host: String) -> G4UserImportMessage {
        G4UserImportMessage { username, id, host }
    }
}
