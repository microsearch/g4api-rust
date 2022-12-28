# \InternalApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_claim_put**](InternalApi.md#user_claim_put) | **PUT** /user-claim | Assign credentials to an anonymous account
[**user_claim_tokens_get**](InternalApi.md#user_claim_tokens_get) | **GET** /user-claim-tokens | Request user claim tokens
[**user_details_id_get**](InternalApi.md#user_details_id_get) | **GET** /user-details/{id} | Get user details by user id
[**user_password_put**](InternalApi.md#user_password_put) | **PUT** /user-password | Reset a user password
[**user_reset_tokens_get**](InternalApi.md#user_reset_tokens_get) | **GET** /user-reset-tokens | Request user password reset tokens



## user_claim_put

> user_claim_put(x_g4_tenant, user_claim_account_request, x_g4_application)
Assign credentials to an anonymous account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**user_claim_account_request** | [**UserClaimAccountRequest**](UserClaimAccountRequest.md) |  | [required] |
**x_g4_application** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_claim_tokens_get

> Vec<crate::models::SecurityToken> user_claim_tokens_get(x_g4_tenant, x_g4_application, email)
Request user claim tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**x_g4_application** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SecurityToken>**](SecurityToken.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_details_id_get

> crate::models::GetUserDetailsResponse user_details_id_get(x_g4_tenant, id)
Get user details by user id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | User id | [required] |

### Return type

[**crate::models::GetUserDetailsResponse**](GetUserDetailsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_password_put

> user_password_put(x_g4_tenant, user_reset_password_request, x_g4_application)
Reset a user password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**user_reset_password_request** | [**UserResetPasswordRequest**](UserResetPasswordRequest.md) |  | [required] |
**x_g4_application** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_reset_tokens_get

> Vec<crate::models::SecurityToken> user_reset_tokens_get(x_g4_tenant, x_g4_application, email)
Request user password reset tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**x_g4_application** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::SecurityToken>**](SecurityToken.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

