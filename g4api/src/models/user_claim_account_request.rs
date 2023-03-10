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
pub struct UserClaimAccountRequest {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UserClaimAccountRequest {
    pub fn new(token: String) -> UserClaimAccountRequest {
        UserClaimAccountRequest {
            token,
            username: None,
            password: None,
        }
    }
}
