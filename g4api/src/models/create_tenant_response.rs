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
pub struct CreateTenantResponse {
    #[serde(rename = "id")]
    pub id: i32,
}

impl CreateTenantResponse {
    pub fn new(id: i32) -> CreateTenantResponse {
        CreateTenantResponse { id }
    }
}