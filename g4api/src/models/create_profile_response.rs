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
pub struct CreateProfileResponse {
    #[serde(rename = "id")]
    pub id: i32,
}

impl CreateProfileResponse {
    pub fn new(id: i32) -> CreateProfileResponse {
        CreateProfileResponse { id }
    }
}
