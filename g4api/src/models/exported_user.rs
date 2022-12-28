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
pub struct ExportedUser {
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "status")]
    pub status: crate::models::UserStatus,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "fullname")]
    pub fullname: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
    #[serde(rename = "profiles")]
    pub profiles: Vec<String>,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "denyCollections")]
    pub deny_collections: Vec<String>,
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    #[serde(rename = "archived")]
    pub archived: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: ::std::collections::HashMap<
        String,
        ::std::collections::HashMap<String, serde_json::Value>,
    >,
}

impl ExportedUser {
    pub fn new(
        created: String,
        status: crate::models::UserStatus,
        username: String,
        fullname: String,
        email: String,
        roles: Vec<String>,
        profiles: Vec<String>,
        collections: Vec<String>,
        deny_collections: Vec<String>,
        password_hash: String,
        archived: Option<String>,
        metadata: ::std::collections::HashMap<
            String,
            ::std::collections::HashMap<String, serde_json::Value>,
        >,
    ) -> ExportedUser {
        ExportedUser {
            created,
            status,
            username,
            fullname,
            email,
            roles,
            profiles,
            collections,
            deny_collections,
            password_hash,
            archived,
            metadata,
        }
    }
}
