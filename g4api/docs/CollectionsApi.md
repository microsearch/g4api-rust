# \CollectionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**collection_contents_documents_post**](CollectionsApi.md#collection_contents_documents_post) | **POST** /collection-contents-documents | Get TOC node documents for collection
[**collection_contents_post**](CollectionsApi.md#collection_contents_post) | **POST** /collection-contents | Get TOC tree for collection
[**collection_metadata_id_get**](CollectionsApi.md#collection_metadata_id_get) | **GET** /collection-metadata/{id} | Get collection metadata
[**collection_metadata_id_put**](CollectionsApi.md#collection_metadata_id_put) | **PUT** /collection-metadata/{id} | Set collection metadata
[**collection_post**](CollectionsApi.md#collection_post) | **POST** /collection | Add/Update documents in a collection
[**collections_get**](CollectionsApi.md#collections_get) | **GET** /collections | Get list of document collections
[**collections_post**](CollectionsApi.md#collections_post) | **POST** /collections | Create document collection



## collection_contents_documents_post

> crate::models::CollectionContentsDocumentsResponse collection_contents_documents_post(x_g4_tenant, collection_contents_documents_request)
Get TOC node documents for collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**collection_contents_documents_request** | Option<[**CollectionContentsDocumentsRequest**](CollectionContentsDocumentsRequest.md)> | Collection contents documents request |  |

### Return type

[**crate::models::CollectionContentsDocumentsResponse**](CollectionContentsDocumentsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collection_contents_post

> crate::models::CollectionContentsResponse collection_contents_post(x_g4_tenant, collection_contents_request)
Get TOC tree for collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**collection_contents_request** | Option<[**CollectionContentsRequest**](CollectionContentsRequest.md)> | Collection contents request |  |

### Return type

[**crate::models::CollectionContentsResponse**](CollectionContentsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collection_metadata_id_get

> ::std::collections::HashMap<String, serde_json::Value> collection_metadata_id_get(x_g4_tenant, id, x_g4_application, app)
Get collection metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Collection id | [required] |
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


## collection_metadata_id_put

> collection_metadata_id_put(x_g4_tenant, id, request_body, x_g4_application, app)
Set collection metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**id** | **i32** | Collection id | [required] |
**request_body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Collection metadata | [required] |
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


## collection_post

> crate::models::UpdateCollectionResponse collection_post(x_g4_tenant, update_collection_request)
Add/Update documents in a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**update_collection_request** | Option<[**UpdateCollectionRequest**](UpdateCollectionRequest.md)> | Update document collection request |  |

### Return type

[**crate::models::UpdateCollectionResponse**](UpdateCollectionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collections_get

> crate::models::GetCollectionsResponse collections_get(x_g4_tenant)
Get list of document collections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |

### Return type

[**crate::models::GetCollectionsResponse**](GetCollectionsResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collections_post

> crate::models::CreateCollectionResponse collections_post(x_g4_tenant, create_collection_request)
Create document collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_g4_tenant** | **String** | Tenant name | [required] |
**create_collection_request** | Option<[**CreateCollectionRequest**](CreateCollectionRequest.md)> | Create document collection request |  |

### Return type

[**crate::models::CreateCollectionResponse**](CreateCollectionResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/_*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

