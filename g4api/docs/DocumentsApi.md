# \DocumentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**document_id_post**](DocumentsApi.md#document_id_post) | **POST** /document/{id} | Verify an uploaded document
[**documents_post**](DocumentsApi.md#documents_post) | **POST** /documents | Prepare for document upload
[**htmlfile_tenant_session_id_signature_filename_get**](DocumentsApi.md#htmlfile_tenant_session_id_signature_filename_get) | **GET** /htmlfile/{tenant}/{sessionId}/{signature}/{filename} | Retrieve raw HTML file from repository



## document_id_post

> document_id_post(x_g4_tenant, id, jobid)
Verify an uploaded document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i64** | Document Id | [required] |
**jobid** | Option<**String**> | Job Id |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_post

> crate::models::LoadDocumentResponse documents_post(x_g4_tenant, x_g4_application, load_document_request)
Prepare for document upload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**x_g4_application** | Option<**String**> | Application name |  |
**load_document_request** | Option<[**LoadDocumentRequest**](LoadDocumentRequest.md)> | Document upload request |  |

### Return type

[**crate::models::LoadDocumentResponse**](LoadDocumentResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## htmlfile_tenant_session_id_signature_filename_get

> htmlfile_tenant_session_id_signature_filename_get(tenant, session_id, signature, filename)
Retrieve raw HTML file from repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Tenant | [required] |
**session_id** | **String** | Session Id | [required] |
**signature** | **String** | Document signature | [required] |
**filename** | **String** | Filename portion of key | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

