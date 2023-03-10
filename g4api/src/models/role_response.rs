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
pub struct RoleResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "claims")]
    pub claims: Vec<String>,
}

impl RoleResponse {
    pub fn new(
        id: i32,
        scope: String,
        name: String,
        claims: Vec<String>,
    ) -> RoleResponse {
        RoleResponse {
            id,
            scope,
            name,
            claims,
        }
    }
}
