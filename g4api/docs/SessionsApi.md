# \SessionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**session_id_delete**](SessionsApi.md#session_id_delete) | **DELETE** /session/{id} | Delete session
[**session_id_get**](SessionsApi.md#session_id_get) | **GET** /session/{id} | Get session
[**session_id_put**](SessionsApi.md#session_id_put) | **PUT** /session/{id} | Update session data
[**session_post**](SessionsApi.md#session_post) | **POST** /session | Create a persistent session
[**static_session_id_get**](SessionsApi.md#static_session_id_get) | **GET** /static-session/{id} | Get session without updating ttl (internal use)



## session_id_delete

> session_id_delete(x_g4_tenant, id)
Delete session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_id_get

> crate::models::GetSessionResponse session_id_get(x_g4_tenant, id)
Get session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetSessionResponse**](GetSessionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_id_put

> session_id_put(x_g4_tenant, id, body)
Update session data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_post

> crate::models::AuthenticatedSessionResponse session_post(x_g4_tenant, create_session_request, x_g4_application)
Create a persistent session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**create_session_request** | [**CreateSessionRequest**](CreateSessionRequest.md) |  | [required] |
**x_g4_application** | Option<**String**> |  |  |

### Return type

[**crate::models::AuthenticatedSessionResponse**](AuthenticatedSessionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## static_session_id_get

> crate::models::GetSessionResponse static_session_id_get(x_g4_tenant, id)
Get session without updating ttl (internal use)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetSessionResponse**](GetSessionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

