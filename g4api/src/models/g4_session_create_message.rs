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
pub struct G4SessionCreateMessage {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "host")]
    pub host: String,
}

impl G4SessionCreateMessage {
    pub fn new(
        session_id: String,
        username: String,
        host: String,
    ) -> G4SessionCreateMessage {
        G4SessionCreateMessage {
            session_id,
            username,
            host,
        }
    }
}
