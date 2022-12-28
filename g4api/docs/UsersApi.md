# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tenant_metadata_get**](UsersApi.md#tenant_metadata_get) | **GET** /tenant-metadata | Get tenant metadata
[**tenant_metadata_put**](UsersApi.md#tenant_metadata_put) | **PUT** /tenant-metadata | Set tenant metadata
[**user_events_post**](UsersApi.md#user_events_post) | **POST** /user-events | Get user events
[**user_id_delete**](UsersApi.md#user_id_delete) | **DELETE** /user/{id} | Archive a user
[**user_id_get**](UsersApi.md#user_id_get) | **GET** /user/{id} | Get user by user id
[**user_id_put**](UsersApi.md#user_id_put) | **PUT** /user/{id} | Update existing user
[**user_import_post**](UsersApi.md#user_import_post) | **POST** /user-import | Import a new user (DEPRECATED: use POST /import-users instead)
[**user_metadata_id_get**](UsersApi.md#user_metadata_id_get) | **GET** /user-metadata/{id} | Get user metadata
[**user_metadata_id_put**](UsersApi.md#user_metadata_id_put) | **PUT** /user-metadata/{id} | Set user metadata
[**user_post**](UsersApi.md#user_post) | **POST** /user | Create new user
[**users_get**](UsersApi.md#users_get) | **GET** /users | Get user list (DEPRECATED: use POST /users instead)
[**users_post**](UsersApi.md#users_post) | **POST** /users | Get user list



## tenant_metadata_get

> ::std::collections::HashMap<String, serde_json::Value> tenant_metadata_get(x_g4_tenant, x_g4_application, app)
Get tenant metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
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


## tenant_metadata_put

> tenant_metadata_put(x_g4_tenant, request_body, x_g4_application, app)
Set tenant metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**request_body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Tenant metadata | [required] |
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


## user_events_post

> crate::models::GetUserEventsResponse user_events_post(x_g4_tenant, get_user_events_request)
Get user events

Returns a list of user events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**get_user_events_request** | [**GetUserEventsRequest**](GetUserEventsRequest.md) | Request parameters | [required] |

### Return type

[**crate::models::GetUserEventsResponse**](GetUserEventsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_delete

> user_id_delete(x_g4_tenant, id, x_g4_application)
Archive a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**id** | **i32** |  | [required] |
**x_g4_application** | Option<**String**> | Application name |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_get

> crate::models::GetUserResponse user_id_get(x_g4_tenant, id, x_g4_application)
Get user by user id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | User id | [required] |
**x_g4_application** | Option<**String**> | Application name |  |

### Return type

[**crate::models::GetUserResponse**](GetUserResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_put

> crate::models::UpdateUserResponse user_id_put(x_g4_tenant, id, update_user_request, x_g4_application)
Update existing user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | User id | [required] |
**update_user_request** | [**UpdateUserRequest**](UpdateUserRequest.md) | Update user request parameters | [required] |
**x_g4_application** | Option<**String**> | Application name |  |

### Return type

[**crate::models::UpdateUserResponse**](UpdateUserResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_import_post

> crate::models::ImportUserResponse user_import_post(x_g4_tenant, x_g4_application, import_user_request)
Import a new user (DEPRECATED: use POST /import-users instead)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**x_g4_application** | Option<**String**> | Application name |  |
**import_user_request** | Option<[**ImportUserRequest**](ImportUserRequest.md)> | Import user parameters |  |

### Return type

[**crate::models::ImportUserResponse**](ImportUserResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_metadata_id_get

> ::std::collections::HashMap<String, serde_json::Value> user_metadata_id_get(x_g4_tenant, id, x_g4_application, app)
Get user metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | User id | [required] |
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


## user_metadata_id_put

> user_metadata_id_put(x_g4_tenant, id, request_body, x_g4_application, app)
Set user metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | User id | [required] |
**request_body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | User metadata | [required] |
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


## user_post

> crate::models::CreateUserResponse user_post(x_g4_tenant, create_user_request, x_g4_application)
Create new user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) | Request parameters | [required] |
**x_g4_application** | Option<**String**> | Application name |  |

### Return type

[**crate::models::CreateUserResponse**](CreateUserResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get

> crate::models::GetUsersWithAppMetadataResponse users_get(x_g4_tenant, x_g4_application, contains, skip, take, archived)
Get user list (DEPRECATED: use POST /users instead)

Returns the list of user records which match the specified (optional) criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**x_g4_application** | Option<**String**> | Application name |  |
**contains** | Option<**String**> | Returns only users whose username, email or fullname contains a string (case insensitive, default: null = match all) |  |
**skip** | Option<**i32**> | Number of users to skip (default: 0) |  |
**take** | Option<**i32**> | Maximum number of users to return (default: all users) |  |
**archived** | Option<**bool**> | Return archived users |  |

### Return type

[**crate::models::GetUsersWithAppMetadataResponse**](GetUsersWithAppMetadataResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_post

> crate::models::GetUsersWithAppMetadataResponse users_post(x_g4_tenant, x_g4_application, get_users_request)
Get user list

Returns the list of user records which match the specified (optional) criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**x_g4_application** | Option<**String**> | Application name |  |
**get_users_request** | Option<[**GetUsersRequest**](GetUsersRequest.md)> | Request parameters |  |

### Return type

[**crate::models::GetUsersWithAppMetadataResponse**](GetUsersWithAppMetadataResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

