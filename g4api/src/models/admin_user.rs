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
pub struct AdminUser {
    #[serde(rename = "id")]
    pub id: i32,
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
    pub roles: Vec<i32>,
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<String>,
    #[serde(rename = "roleNames")]
    pub role_names: Vec<String>,
}

impl AdminUser {
    pub fn new(
        id: i32,
        created: String,
        status: crate::models::UserStatus,
        username: String,
        fullname: String,
        email: String,
        roles: Vec<i32>,
        last_seen: Option<String>,
        role_names: Vec<String>,
    ) -> AdminUser {
        AdminUser {
            id,
            created,
            status,
            username,
            fullname,
            email,
            roles,
            last_seen,
            role_names,
        }
    }
}