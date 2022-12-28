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
pub struct CollectionNode {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "key")]
    pub key: Vec<String>,
    #[serde(rename = "children")]
    pub children: Option<Vec<crate::models::CollectionNode>>,
}

impl CollectionNode {
    pub fn new(
        value: String,
        key: Vec<String>,
        children: Option<Vec<crate::models::CollectionNode>>,
    ) -> CollectionNode {
        CollectionNode {
            value,
            key,
            children,
        }
    }
}