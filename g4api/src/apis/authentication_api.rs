/*
 * G4 API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `auth_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthGetError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `auth_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthPostError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `password_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PasswordPutError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `policy_password_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolicyPasswordGetError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `sync_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncPostError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

pub fn auth_get(
    configuration: &configuration::Configuration,
) -> Result<crate::models::RefreshResponse, Error<AuthGetError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/auth", configuration.base_path);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn auth_post(
    configuration: &configuration::Configuration,
    user_authentication_request: crate::models::UserAuthenticationRequest,
    x_g4_tenant: Option<&str>,
    x_g4_application: Option<&str>,
) -> Result<crate::models::UserAuthenticationResponse, Error<AuthPostError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/auth", configuration.base_path);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_g4_tenant {
        local_var_req_builder = local_var_req_builder
            .header("x-g4-tenant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_g4_application {
        local_var_req_builder = local_var_req_builder
            .header("x-g4-application", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder =
        local_var_req_builder.json(&user_authentication_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn password_put(
    configuration: &configuration::Configuration,
    password_change_request: crate::models::PasswordChangeRequest,
    x_g4_tenant: Option<&str>,
    x_g4_application: Option<&str>,
) -> Result<(), Error<PasswordPutError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/password", configuration.base_path);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_g4_tenant {
        local_var_req_builder = local_var_req_builder
            .header("x-g4-tenant", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_g4_application {
        local_var_req_builder = local_var_req_builder
            .header("x-g4-application", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder =
        local_var_req_builder.json(&password_change_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        Ok(())
    } else {
        let local_var_entity: Option<PasswordPutError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn policy_password_get(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
) -> Result<crate::models::PasswordPolicy, Error<PolicyPasswordGetError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str =
        format!("{}/policy/password", configuration.base_path);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("x-g4-tenant", x_g4_tenant.to_string());
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PolicyPasswordGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn sync_post(
    configuration: &configuration::Configuration,
) -> Result<(), Error<SyncPostError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/sync", configuration.base_path);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        Ok(())
    } else {
        let local_var_entity: Option<SyncPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
