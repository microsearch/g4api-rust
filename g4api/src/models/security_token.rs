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
pub struct SecurityToken {
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(rename = "fullname")]
    pub fullname: String,
    #[serde(rename = "token")]
    pub token: String,
}

impl SecurityToken {
    pub fn new(
        username: Option<String>,
        fullname: String,
        token: String,
    ) -> SecurityToken {
        SecurityToken {
            username,
            fullname,
            token,
        }
    }
}