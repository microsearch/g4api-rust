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
pub struct UserAuthenticationResponse {
    #[serde(rename = "validCredentials")]
    pub valid_credentials: bool,
    #[serde(rename = "accessAllowed")]
    pub access_allowed: bool,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(rename = "fullname")]
    pub fullname: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "bearer")]
    pub bearer: Option<String>,
    #[serde(rename = "claims")]
    pub claims: Option<Vec<String>>,
    #[serde(rename = "roles")]
    pub roles: Option<Vec<String>>,
    #[serde(rename = "profiles")]
    pub profiles: Option<Vec<String>>,
    #[serde(rename = "version")]
    pub version: String,
}

impl UserAuthenticationResponse {
    pub fn new(
        valid_credentials: bool,
        access_allowed: bool,
        user_id: Option<i32>,
        username: Option<String>,
        fullname: Option<String>,
        email: Option<String>,
        bearer: Option<String>,
        claims: Option<Vec<String>>,
        roles: Option<Vec<String>>,
        profiles: Option<Vec<String>>,
        version: String,
    ) -> UserAuthenticationResponse {
        UserAuthenticationResponse {
            valid_credentials,
            access_allowed,
            user_id,
            username,
            fullname,
            email,
            bearer,
            claims,
            roles,
            profiles,
            version,
        }
    }
}
