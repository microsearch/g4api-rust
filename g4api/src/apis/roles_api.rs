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

/// struct for typed errors of method `role_id_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleIdDeleteError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleIdGetError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_id_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleIdPutError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_metadata_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleMetadataIdGetError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_metadata_id_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleMetadataIdPutError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolePostError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `roles_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesGetError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `roles_scope_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RolesScopeGetError {
    Status400(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

pub fn role_id_delete(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
    id: i32,
) -> Result<(), Error<RoleIdDeleteError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str =
        format!("{}/role/{id}", configuration.base_path, id = id);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<RoleIdDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn role_id_get(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
    id: i32,
) -> Result<crate::models::RoleResponse, Error<RoleIdGetError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str =
        format!("{}/role/{id}", configuration.base_path, id = id);
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
        let local_var_entity: Option<RoleIdGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn role_id_put(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
    id: i32,
    update_role_request: Option<crate::models::UpdateRoleRequest>,
) -> Result<crate::models::RoleResponse, Error<RoleIdPutError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str =
        format!("{}/role/{id}", configuration.base_path, id = id);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&update_role_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RoleIdPutError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn role_metadata_id_get(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
    id: i32,
    x_g4_application: Option<&str>,
    app: Option<&str>,
) -> Result<
    ::std::collections::HashMap<String, serde_json::Value>,
    Error<RoleMetadataIdGetError>,
> {
    let local_var_client = &configuration.client;

    let local_var_uri_str =
        format!("{}/role-metadata/{id}", configuration.base_path, id = id);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = app {
        local_var_req_builder =
            local_var_req_builder.query(&[("app", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("x-g4-tenant", x_g4_tenant.to_string());
    if let Some(local_var_param_value) = x_g4_application {
        local_var_req_builder = local_var_req_builder
            .header("x-g4-application", local_var_param_value.to_string());
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
        let local_var_entity: Option<RoleMetadataIdGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn role_metadata_id_put(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
    id: i32,
    request_body: ::std::collections::HashMap<String, serde_json::Value>,
    x_g4_application: Option<&str>,
    app: Option<&str>,
) -> Result<(), Error<RoleMetadataIdPutError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str =
        format!("{}/role-metadata/{id}", configuration.base_path, id = id);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = app {
        local_var_req_builder =
            local_var_req_builder.query(&[("app", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder
            .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("x-g4-tenant", x_g4_tenant.to_string());
    if let Some(local_var_param_value) = x_g4_application {
        local_var_req_builder = local_var_req_builder
            .header("x-g4-application", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        Ok(())
    } else {
        let local_var_entity: Option<RoleMetadataIdPutError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn role_post(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
    create_role_request: Option<crate::models::CreateRoleRequest>,
) -> Result<crate::models::CreateRoleResponse, Error<RolePostError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/role", configuration.base_path);
    let mut local_var_req_builder = local_var_client
        .request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&create_role_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error()
        && !local_var_status.is_server_error()
    {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RolePostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn roles_get(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
) -> Result<crate::models::GetRolesResponse, Error<RolesGetError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/roles", configuration.base_path);
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
        let local_var_entity: Option<RolesGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn roles_scope_get(
    configuration: &configuration::Configuration,
    x_g4_tenant: &str,
    scope: &str,
) -> Result<crate::models::GetRolesResponse, Error<RolesScopeGetError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/roles/{scope}",
        configuration.base_path,
        scope = crate::apis::urlencode(scope)
    );
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
        let local_var_entity: Option<RolesScopeGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
