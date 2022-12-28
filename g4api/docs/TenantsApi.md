# \TenantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tenant_id_delete**](TenantsApi.md#tenant_id_delete) | **DELETE** /tenant/{id} | Archive a tenant
[**tenant_id_get**](TenantsApi.md#tenant_id_get) | **GET** /tenant/{id} | Get tenant by id
[**tenant_post**](TenantsApi.md#tenant_post) | **POST** /tenant | Create a new tenant
[**tenants_delete**](TenantsApi.md#tenants_delete) | **DELETE** /tenants | Purge archived tenants
[**tenants_get**](TenantsApi.md#tenants_get) | **GET** /tenants | Get tenant list



## tenant_id_delete

> tenant_id_delete(id)
Archive a tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Tenant id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenant_id_get

> crate::models::GetTenantResponse tenant_id_get(id)
Get tenant by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Tenant id | [required] |

### Return type

[**crate::models::GetTenantResponse**](GetTenantResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenant_post

> crate::models::CreateTenantResponse tenant_post(create_tenant_request)
Create a new tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tenant_request** | Option<[**CreateTenantRequest**](CreateTenantRequest.md)> | Create tenant request |  |

### Return type

[**crate::models::CreateTenantResponse**](CreateTenantResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_delete

> tenants_delete(days)
Purge archived tenants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Minimum age in days |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_get

> crate::models::GetTenantsResponse tenants_get()
Get tenant list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetTenantsResponse**](GetTenantsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

