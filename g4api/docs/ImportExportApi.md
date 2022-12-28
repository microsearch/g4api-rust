# \ImportExportApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_users_post**](ImportExportApi.md#export_users_post) | **POST** /export-users | Export users
[**import_users_post**](ImportExportApi.md#import_users_post) | **POST** /import-users | Import users



## export_users_post

> crate::models::ExportUsersResponse export_users_post(x_g4_tenant)
Export users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |

### Return type

[**crate::models::ExportUsersResponse**](ExportUsersResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_users_post

> crate::models::ImportUsersResponse import_users_post(import_users_request, x_g4_application)
Import users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_users_request** | [**ImportUsersRequest**](ImportUsersRequest.md) | Exported user list | [required] |
**x_g4_application** | Option<**String**> | Application name |  |

### Return type

[**crate::models::ImportUsersResponse**](ImportUsersResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

