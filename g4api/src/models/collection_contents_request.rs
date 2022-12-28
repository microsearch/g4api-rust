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
pub struct CollectionContentsRequest {
    #[serde(rename = "collectionId")]
    pub collection_id: i32,
    #[serde(rename = "contentsFieldnames")]
    pub contents_fieldnames: Vec<String>,
}

impl CollectionContentsRequest {
    pub fn new(
        collection_id: i32,
        contents_fieldnames: Vec<String>,
    ) -> CollectionContentsRequest {
        CollectionContentsRequest {
            collection_id,
            contents_fieldnames,
        }
    }
}
