# \RolesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**role_id_delete**](RolesApi.md#role_id_delete) | **DELETE** /role/{id} | Archive a role
[**role_id_get**](RolesApi.md#role_id_get) | **GET** /role/{id} | Get a role by id
[**role_id_put**](RolesApi.md#role_id_put) | **PUT** /role/{id} | Update a role
[**role_metadata_id_get**](RolesApi.md#role_metadata_id_get) | **GET** /role-metadata/{id} | Get role metadata
[**role_metadata_id_put**](RolesApi.md#role_metadata_id_put) | **PUT** /role-metadata/{id} | Set role metadata
[**role_post**](RolesApi.md#role_post) | **POST** /role | Create a new role
[**roles_get**](RolesApi.md#roles_get) | **GET** /roles | Get role list for a tenant
[**roles_scope_get**](RolesApi.md#roles_scope_get) | **GET** /roles/{scope} | Get role list for a tenant and scope



## role_id_delete

> role_id_delete(x_g4_tenant, id)
Archive a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Role id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_id_get

> crate::models::RoleResponse role_id_get(x_g4_tenant, id)
Get a role by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Role id | [required] |

### Return type

[**crate::models::RoleResponse**](RoleResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_id_put

> crate::models::RoleResponse role_id_put(x_g4_tenant, id, update_role_request)
Update a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Role id | [required] |
**update_role_request** | Option<[**UpdateRoleRequest**](UpdateRoleRequest.md)> | Update role request |  |

### Return type

[**crate::models::RoleResponse**](RoleResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_metadata_id_get

> ::std::collections::HashMap<String, serde_json::Value> role_metadata_id_get(x_g4_tenant, id, x_g4_application, app)
Get role metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Role id | [required] |
**x_g4_application** | Option<**String**> | Application name |  |
**app** | Option<**String**> | Override x-g4-application |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_metadata_id_put

> role_metadata_id_put(x_g4_tenant, id, request_body, x_g4_application, app)
Set role metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Role id | [required] |
**request_body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Role metadata | [required] |
**x_g4_application** | Option<**String**> | Application name |  |
**app** | Option<**String**> | Override x-g4-application |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_post

> crate::models::CreateRoleResponse role_post(x_g4_tenant, create_role_request)
Create a new role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**create_role_request** | Option<[**CreateRoleRequest**](CreateRoleRequest.md)> | Create role request |  |

### Return type

[**crate::models::CreateRoleResponse**](CreateRoleResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## roles_get

> crate::models::GetRolesResponse roles_get(x_g4_tenant)
Get role list for a tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |

### Return type

[**crate::models::GetRolesResponse**](GetRolesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## roles_scope_get

> crate::models::GetRolesResponse roles_scope_get(x_g4_tenant, scope)
Get role list for a tenant and scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**scope** | **String** | Scope | [required] |

### Return type

[**crate::models::GetRolesResponse**](GetRolesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

