# UpdateUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**crate::models::UserStatus**](UserStatus.md)> |  | [optional]
**username** | Option<**String**> |  | [optional]
**fullname** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**scope** | Option<**String**> |  | [optional]
**roles** | Option<**Vec<i32>**> |  | [optional]
**profiles** | Option<**Vec<i32>**> |  | [optional]
**collections** | Option<**Vec<i32>**> |  | [optional]
**deny_collections** | Option<**Vec<i32>**> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**password** | Option<**String**> |  | [optional]
**password_crypto** | Option<**String**> |  | [optional]
**password_salt** | Option<**String**> |  | [optional]
**password_hash** | Option<**String**> |  | [optional]
**app_name** | Option<**String**> |  | [optional]
**app_metadata** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


