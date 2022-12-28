# \AuthenticationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_get**](AuthenticationApi.md#auth_get) | **GET** /auth | Refresh authentication token
[**auth_post**](AuthenticationApi.md#auth_post) | **POST** /auth | Authenticate user credentials
[**password_put**](AuthenticationApi.md#password_put) | **PUT** /password | Change password
[**policy_password_get**](AuthenticationApi.md#policy_password_get) | **GET** /policy/password | Get a tenant's password policy
[**sync_post**](AuthenticationApi.md#sync_post) | **POST** /sync | Process any pending G3 account synchronization requests



## auth_get

> crate::models::RefreshResponse auth_get()
Refresh authentication token

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RefreshResponse**](RefreshResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_post

> crate::models::UserAuthenticationResponse auth_post(user_authentication_request, x_g4_tenant, x_g4_application)
Authenticate user credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_authentication_request** | [**UserAuthenticationRequest**](UserAuthenticationRequest.md) | Authentication request | [required] |
**x_g4_tenant** | Option<**String**> | Tenant name |  |
**x_g4_application** | Option<**String**> | Application name |  |

### Return type

[**crate::models::UserAuthenticationResponse**](UserAuthenticationResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## password_put

> password_put(password_change_request, x_g4_tenant, x_g4_application)
Change password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_change_request** | [**PasswordChangeRequest**](PasswordChangeRequest.md) | Password Change Request | [required] |
**x_g4_tenant** | Option<**String**> | Tenant name |  |
**x_g4_application** | Option<**String**> | Application name |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policy_password_get

> crate::models::PasswordPolicy policy_password_get(x_g4_tenant)
Get a tenant's password policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |

### Return type

[**crate::models::PasswordPolicy**](PasswordPolicy.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_post

> sync_post()
Process any pending G3 account synchronization requests

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

