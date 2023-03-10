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
pub struct UserAuthenticationRequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl UserAuthenticationRequest {
    pub fn new(
        username: String,
        password: String,
    ) -> UserAuthenticationRequest {
        UserAuthenticationRequest {
            username,
            password,
            detail: None,
        }
    }
}
