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
pub struct UpdateProfileResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "collections")]
    pub collections: Vec<i32>,
}

impl UpdateProfileResponse {
    pub fn new(
        id: i32,
        name: String,
        collections: Vec<i32>,
    ) -> UpdateProfileResponse {
        UpdateProfileResponse {
            id,
            name,
            collections,
        }
    }
}
