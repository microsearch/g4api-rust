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
pub struct CollectionContentsDocument {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "docMetadata")]
    pub doc_metadata: ::std::collections::HashMap<String, String>,
}

impl CollectionContentsDocument {
    pub fn new(
        title: String,
        signature: String,
        doc_metadata: ::std::collections::HashMap<String, String>,
    ) -> CollectionContentsDocument {
        CollectionContentsDocument {
            title,
            signature,
            doc_metadata,
        }
    }
}
