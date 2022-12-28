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
pub struct UpdateUserResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "fullname")]
    pub fullname: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "roles")]
    pub roles: Vec<i32>,
    #[serde(rename = "profiles")]
    pub profiles: Vec<i32>,
    #[serde(rename = "collections")]
    pub collections: Vec<i32>,
    #[serde(rename = "denyCollections")]
    pub deny_collections: Vec<i32>,
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata:
        Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl UpdateUserResponse {
    pub fn new(
        id: i32,
        status: crate::models::UserStatus,
        username: String,
        fullname: String,
        email: String,
        roles: Vec<i32>,
        profiles: Vec<i32>,
        collections: Vec<i32>,
        deny_collections: Vec<i32>,
        last_seen: Option<String>,
        metadata: Option<
            ::std::collections::HashMap<String, serde_json::Value>,
        >,
    ) -> UpdateUserResponse {
        UpdateUserResponse {
            id,
            status,
            username,
            fullname,
            email,
            roles,
            profiles,
            collections,
            deny_collections,
            last_seen,
            metadata,
        }
    }
}