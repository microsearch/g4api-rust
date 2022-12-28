# \AdminsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_post**](AdminsApi.md#admin_post) | **POST** /admin | Create new admin user
[**admins_get**](AdminsApi.md#admins_get) | **GET** /admins | Get admin user list



## admin_post

> crate::models::CreateAdminResponse admin_post(create_admin_request)
Create new admin user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_admin_request** | [**CreateAdminRequest**](CreateAdminRequest.md) | Create admin user request | [required] |

### Return type

[**crate::models::CreateAdminResponse**](CreateAdminResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admins_get

> crate::models::GetAdminsResponse admins_get()
Get admin user list

Returns the list of all admin user records.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAdminsResponse**](GetAdminsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

