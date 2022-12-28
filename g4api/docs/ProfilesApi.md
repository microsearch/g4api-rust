# \ProfilesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**profile_id_delete**](ProfilesApi.md#profile_id_delete) | **DELETE** /profile/{id} | Archive a profile
[**profile_id_get**](ProfilesApi.md#profile_id_get) | **GET** /profile/{id} | Get a profile by id
[**profile_id_put**](ProfilesApi.md#profile_id_put) | **PUT** /profile/{id} | Update a profile
[**profile_metadata_id_get**](ProfilesApi.md#profile_metadata_id_get) | **GET** /profile-metadata/{id} | Get profile metadata
[**profile_metadata_id_put**](ProfilesApi.md#profile_metadata_id_put) | **PUT** /profile-metadata/{id} | Set profile metadata
[**profile_post**](ProfilesApi.md#profile_post) | **POST** /profile | Create a new profile
[**profiles_get**](ProfilesApi.md#profiles_get) | **GET** /profiles | Get profile list for a tenant



## profile_id_delete

> profile_id_delete(x_g4_tenant, id)
Archive a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Profile id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_id_get

> profile_id_get(id, x_g4_tenant)
Get a profile by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Profile id | [required] |
**x_g4_tenant** | **String** | Tenant name | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_id_put

> crate::models::UpdateProfileResponse profile_id_put(x_g4_tenant, id, update_profile_request)
Update a profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Profile id | [required] |
**update_profile_request** | Option<[**UpdateProfileRequest**](UpdateProfileRequest.md)> | Update profile request |  |

### Return type

[**crate::models::UpdateProfileResponse**](UpdateProfileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_metadata_id_get

> ::std::collections::HashMap<String, serde_json::Value> profile_metadata_id_get(x_g4_tenant, id, x_g4_application, app)
Get profile metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Profile id | [required] |
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


## profile_metadata_id_put

> profile_metadata_id_put(x_g4_tenant, id, request_body, x_g4_application, app)
Set profile metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Profile id | [required] |
**request_body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Profile metadata | [required] |
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


## profile_post

> crate::models::CreateProfileResponse profile_post(x_g4_tenant, create_profile_request)
Create a new profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**create_profile_request** | Option<[**CreateProfileRequest**](CreateProfileRequest.md)> | Create profile request |  |

### Return type

[**crate::models::CreateProfileResponse**](CreateProfileResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_get

> crate::models::GetProfilesResponse profiles_get(x_g4_tenant)
Get profile list for a tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |

### Return type

[**crate::models::GetProfilesResponse**](GetProfilesResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

