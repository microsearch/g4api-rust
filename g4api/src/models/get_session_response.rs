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
pub struct GetSessionResponse {
    #[serde(rename = "bearer")]
    pub bearer: String,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "data")]
    pub data: Option<serde_json::Value>,
}

impl GetSessionResponse {
    pub fn new(
        bearer: String,
        user_id: i32,
        data: Option<serde_json::Value>,
    ) -> GetSessionResponse {
        GetSessionResponse {
            bearer,
            user_id,
            data,
        }
    }
}
