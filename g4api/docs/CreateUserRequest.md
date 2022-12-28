# CreateUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**crate::models::UserStatus**](UserStatus.md) |  | 
**username** | **String** |  | 
**password** | **String** |  | 
**fullname** | **String** |  | 
**email** | **String** |  | 
**roles** | **Vec<i32>** |  | 
**profiles** | **Vec<i32>** |  | 
**collections** | **Vec<i32>** |  | 
**deny_collections** | **Vec<i32>** |  | 
**custom_fields** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**app_name** | Option<**String**> |  | 
**app_metadata** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


