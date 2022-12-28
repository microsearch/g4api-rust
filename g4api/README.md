# Rust API client for g4api

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 1.6.5
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AdminsApi* | [**admin_post**](docs/AdminsApi.md#admin_post) | **POST** /admin | Create new admin user
*AdminsApi* | [**admins_get**](docs/AdminsApi.md#admins_get) | **GET** /admins | Get admin user list
*AuthenticationApi* | [**auth_get**](docs/AuthenticationApi.md#auth_get) | **GET** /auth | Refresh authentication token
*AuthenticationApi* | [**auth_post**](docs/AuthenticationApi.md#auth_post) | **POST** /auth | Authenticate user credentials
*AuthenticationApi* | [**password_put**](docs/AuthenticationApi.md#password_put) | **PUT** /password | Change password
*AuthenticationApi* | [**policy_password_get**](docs/AuthenticationApi.md#policy_password_get) | **GET** /policy/password | Get a tenant's password policy
*AuthenticationApi* | [**sync_post**](docs/AuthenticationApi.md#sync_post) | **POST** /sync | Process any pending G3 account synchronization requests
*CollectionsApi* | [**collection_contents_documents_post**](docs/CollectionsApi.md#collection_contents_documents_post) | **POST** /collection-contents-documents | Get TOC node documents for collection
*CollectionsApi* | [**collection_contents_post**](docs/CollectionsApi.md#collection_contents_post) | **POST** /collection-contents | Get TOC tree for collection
*CollectionsApi* | [**collection_metadata_id_get**](docs/CollectionsApi.md#collection_metadata_id_get) | **GET** /collection-metadata/{id} | Get collection metadata
*CollectionsApi* | [**collection_metadata_id_put**](docs/CollectionsApi.md#collection_metadata_id_put) | **PUT** /collection-metadata/{id} | Set collection metadata
*CollectionsApi* | [**collection_post**](docs/CollectionsApi.md#collection_post) | **POST** /collection | Add/Update documents in a collection
*CollectionsApi* | [**collections_get**](docs/CollectionsApi.md#collections_get) | **GET** /collections | Get list of document collections
*CollectionsApi* | [**collections_post**](docs/CollectionsApi.md#collections_post) | **POST** /collections | Create document collection
*DocumentsApi* | [**document_id_post**](docs/DocumentsApi.md#document_id_post) | **POST** /document/{id} | Verify an uploaded document
*DocumentsApi* | [**documents_post**](docs/DocumentsApi.md#documents_post) | **POST** /documents | Prepare for document upload
*DocumentsApi* | [**htmlfile_tenant_session_id_signature_filename_get**](docs/DocumentsApi.md#htmlfile_tenant_session_id_signature_filename_get) | **GET** /htmlfile/{tenant}/{sessionId}/{signature}/{filename} | Retrieve raw HTML file from repository
*ImportExportApi* | [**export_users_post**](docs/ImportExportApi.md#export_users_post) | **POST** /export-users | Export users
*ImportExportApi* | [**import_users_post**](docs/ImportExportApi.md#import_users_post) | **POST** /import-users | Import users
*InternalApi* | [**user_claim_put**](docs/InternalApi.md#user_claim_put) | **PUT** /user-claim | Assign credentials to an anonymous account
*InternalApi* | [**user_claim_tokens_get**](docs/InternalApi.md#user_claim_tokens_get) | **GET** /user-claim-tokens | Request user claim tokens
*InternalApi* | [**user_details_id_get**](docs/InternalApi.md#user_details_id_get) | **GET** /user-details/{id} | Get user details by user id
*InternalApi* | [**user_password_put**](docs/InternalApi.md#user_password_put) | **PUT** /user-password | Reset a user password
*InternalApi* | [**user_reset_tokens_get**](docs/InternalApi.md#user_reset_tokens_get) | **GET** /user-reset-tokens | Request user password reset tokens
*ProfilesApi* | [**profile_id_delete**](docs/ProfilesApi.md#profile_id_delete) | **DELETE** /profile/{id} | Archive a profile
*ProfilesApi* | [**profile_id_get**](docs/ProfilesApi.md#profile_id_get) | **GET** /profile/{id} | Get a profile by id
*ProfilesApi* | [**profile_id_put**](docs/ProfilesApi.md#profile_id_put) | **PUT** /profile/{id} | Update a profile
*ProfilesApi* | [**profile_metadata_id_get**](docs/ProfilesApi.md#profile_metadata_id_get) | **GET** /profile-metadata/{id} | Get profile metadata
*ProfilesApi* | [**profile_metadata_id_put**](docs/ProfilesApi.md#profile_metadata_id_put) | **PUT** /profile-metadata/{id} | Set profile metadata
*ProfilesApi* | [**profile_post**](docs/ProfilesApi.md#profile_post) | **POST** /profile | Create a new profile
*ProfilesApi* | [**profiles_get**](docs/ProfilesApi.md#profiles_get) | **GET** /profiles | Get profile list for a tenant
*RolesApi* | [**role_id_delete**](docs/RolesApi.md#role_id_delete) | **DELETE** /role/{id} | Archive a role
*RolesApi* | [**role_id_get**](docs/RolesApi.md#role_id_get) | **GET** /role/{id} | Get a role by id
*RolesApi* | [**role_id_put**](docs/RolesApi.md#role_id_put) | **PUT** /role/{id} | Update a role
*RolesApi* | [**role_metadata_id_get**](docs/RolesApi.md#role_metadata_id_get) | **GET** /role-metadata/{id} | Get role metadata
*RolesApi* | [**role_metadata_id_put**](docs/RolesApi.md#role_metadata_id_put) | **PUT** /role-metadata/{id} | Set role metadata
*RolesApi* | [**role_post**](docs/RolesApi.md#role_post) | **POST** /role | Create a new role
*RolesApi* | [**roles_get**](docs/RolesApi.md#roles_get) | **GET** /roles | Get role list for a tenant
*RolesApi* | [**roles_scope_get**](docs/RolesApi.md#roles_scope_get) | **GET** /roles/{scope} | Get role list for a tenant and scope
*SessionsApi* | [**session_id_delete**](docs/SessionsApi.md#session_id_delete) | **DELETE** /session/{id} | Delete session
*SessionsApi* | [**session_id_get**](docs/SessionsApi.md#session_id_get) | **GET** /session/{id} | Get session
*SessionsApi* | [**session_id_put**](docs/SessionsApi.md#session_id_put) | **PUT** /session/{id} | Update session data
*SessionsApi* | [**session_post**](docs/SessionsApi.md#session_post) | **POST** /session | Create a persistent session
*SessionsApi* | [**static_session_id_get**](docs/SessionsApi.md#static_session_id_get) | **GET** /static-session/{id} | Get session without updating ttl (internal use)
*TenantsApi* | [**tenant_id_delete**](docs/TenantsApi.md#tenant_id_delete) | **DELETE** /tenant/{id} | Archive a tenant
*TenantsApi* | [**tenant_id_get**](docs/TenantsApi.md#tenant_id_get) | **GET** /tenant/{id} | Get tenant by id
*TenantsApi* | [**tenant_post**](docs/TenantsApi.md#tenant_post) | **POST** /tenant | Create a new tenant
*TenantsApi* | [**tenants_delete**](docs/TenantsApi.md#tenants_delete) | **DELETE** /tenants | Purge archived tenants
*TenantsApi* | [**tenants_get**](docs/TenantsApi.md#tenants_get) | **GET** /tenants | Get tenant list
*UsersApi* | [**tenant_metadata_get**](docs/UsersApi.md#tenant_metadata_get) | **GET** /tenant-metadata | Get tenant metadata
*UsersApi* | [**tenant_metadata_put**](docs/UsersApi.md#tenant_metadata_put) | **PUT** /tenant-metadata | Set tenant metadata
*UsersApi* | [**user_events_post**](docs/UsersApi.md#user_events_post) | **POST** /user-events | Get user events
*UsersApi* | [**user_id_delete**](docs/UsersApi.md#user_id_delete) | **DELETE** /user/{id} | Archive a user
*UsersApi* | [**user_id_get**](docs/UsersApi.md#user_id_get) | **GET** /user/{id} | Get user by user id
*UsersApi* | [**user_id_put**](docs/UsersApi.md#user_id_put) | **PUT** /user/{id} | Update existing user
*UsersApi* | [**user_import_post**](docs/UsersApi.md#user_import_post) | **POST** /user-import | Import a new user (DEPRECATED: use POST /import-users instead)
*UsersApi* | [**user_metadata_id_get**](docs/UsersApi.md#user_metadata_id_get) | **GET** /user-metadata/{id} | Get user metadata
*UsersApi* | [**user_metadata_id_put**](docs/UsersApi.md#user_metadata_id_put) | **PUT** /user-metadata/{id} | Set user metadata
*UsersApi* | [**user_post**](docs/UsersApi.md#user_post) | **POST** /user | Create new user
*UsersApi* | [**users_get**](docs/UsersApi.md#users_get) | **GET** /users | Get user list (DEPRECATED: use POST /users instead)
*UsersApi* | [**users_post**](docs/UsersApi.md#users_post) | **POST** /users | Get user list


## Documentation For Models

 - [AdminUser](docs/AdminUser.md)
 - [AuthenticatedSessionResponse](docs/AuthenticatedSessionResponse.md)
 - [Collection](docs/Collection.md)
 - [CollectionContentsDocument](docs/CollectionContentsDocument.md)
 - [CollectionContentsDocumentsRequest](docs/CollectionContentsDocumentsRequest.md)
 - [CollectionContentsDocumentsResponse](docs/CollectionContentsDocumentsResponse.md)
 - [CollectionContentsRequest](docs/CollectionContentsRequest.md)
 - [CollectionContentsResponse](docs/CollectionContentsResponse.md)
 - [CollectionNode](docs/CollectionNode.md)
 - [ContentsField](docs/ContentsField.md)
 - [CreateAdminRequest](docs/CreateAdminRequest.md)
 - [CreateAdminResponse](docs/CreateAdminResponse.md)
 - [CreateCollectionRequest](docs/CreateCollectionRequest.md)
 - [CreateCollectionResponse](docs/CreateCollectionResponse.md)
 - [CreateProfileRequest](docs/CreateProfileRequest.md)
 - [CreateProfileResponse](docs/CreateProfileResponse.md)
 - [CreateRoleRequest](docs/CreateRoleRequest.md)
 - [CreateRoleResponse](docs/CreateRoleResponse.md)
 - [CreateSessionRequest](docs/CreateSessionRequest.md)
 - [CreateTenantRequest](docs/CreateTenantRequest.md)
 - [CreateTenantResponse](docs/CreateTenantResponse.md)
 - [CreateUserRequest](docs/CreateUserRequest.md)
 - [CreateUserResponse](docs/CreateUserResponse.md)
 - [Document](docs/Document.md)
 - [DocumentUpdate](docs/DocumentUpdate.md)
 - [ExportUsersResponse](docs/ExportUsersResponse.md)
 - [ExportedUser](docs/ExportedUser.md)
 - [G4AuthAuthMessage](docs/G4AuthAuthMessage.md)
 - [G4CollectionLoadedMessage](docs/G4CollectionLoadedMessage.md)
 - [G4CollectionLoadingMessage](docs/G4CollectionLoadingMessage.md)
 - [G4DocumentLoadedMessage](docs/G4DocumentLoadedMessage.md)
 - [G4SessionCloseMessage](docs/G4SessionCloseMessage.md)
 - [G4SessionCreateMessage](docs/G4SessionCreateMessage.md)
 - [G4SessionFailMessage](docs/G4SessionFailMessage.md)
 - [G4TenantArchiveMessage](docs/G4TenantArchiveMessage.md)
 - [G4TenantCreateMessage](docs/G4TenantCreateMessage.md)
 - [G4UserArchiveMessage](docs/G4UserArchiveMessage.md)
 - [G4UserCreateMessage](docs/G4UserCreateMessage.md)
 - [G4UserImportMessage](docs/G4UserImportMessage.md)
 - [G4UserUpdateMessage](docs/G4UserUpdateMessage.md)
 - [GetAdminsResponse](docs/GetAdminsResponse.md)
 - [GetCollectionsResponse](docs/GetCollectionsResponse.md)
 - [GetProfileResponse](docs/GetProfileResponse.md)
 - [GetProfilesResponse](docs/GetProfilesResponse.md)
 - [GetRolesResponse](docs/GetRolesResponse.md)
 - [GetSessionResponse](docs/GetSessionResponse.md)
 - [GetTenantResponse](docs/GetTenantResponse.md)
 - [GetTenantsResponse](docs/GetTenantsResponse.md)
 - [GetUserDetailsResponse](docs/GetUserDetailsResponse.md)
 - [GetUserEventsRequest](docs/GetUserEventsRequest.md)
 - [GetUserEventsResponse](docs/GetUserEventsResponse.md)
 - [GetUserResponse](docs/GetUserResponse.md)
 - [GetUsersRequest](docs/GetUsersRequest.md)
 - [GetUsersWithAppMetadataResponse](docs/GetUsersWithAppMetadataResponse.md)
 - [ImportUserRequest](docs/ImportUserRequest.md)
 - [ImportUserResponse](docs/ImportUserResponse.md)
 - [ImportUsersRequest](docs/ImportUsersRequest.md)
 - [ImportUsersResponse](docs/ImportUsersResponse.md)
 - [LoadDocumentRequest](docs/LoadDocumentRequest.md)
 - [LoadDocumentResponse](docs/LoadDocumentResponse.md)
 - [PasswordChangeRequest](docs/PasswordChangeRequest.md)
 - [PasswordPolicy](docs/PasswordPolicy.md)
 - [ProblemDetails](docs/ProblemDetails.md)
 - [RefreshResponse](docs/RefreshResponse.md)
 - [RetrievedUserEvent](docs/RetrievedUserEvent.md)
 - [RoleResponse](docs/RoleResponse.md)
 - [SecurityToken](docs/SecurityToken.md)
 - [UpdateCollectionRequest](docs/UpdateCollectionRequest.md)
 - [UpdateCollectionResponse](docs/UpdateCollectionResponse.md)
 - [UpdateProfileRequest](docs/UpdateProfileRequest.md)
 - [UpdateProfileResponse](docs/UpdateProfileResponse.md)
 - [UpdateRoleRequest](docs/UpdateRoleRequest.md)
 - [UpdateStatus](docs/UpdateStatus.md)
 - [UpdateUserRequest](docs/UpdateUserRequest.md)
 - [UpdateUserResponse](docs/UpdateUserResponse.md)
 - [UserAuthenticationRequest](docs/UserAuthenticationRequest.md)
 - [UserAuthenticationResponse](docs/UserAuthenticationResponse.md)
 - [UserClaimAccountRequest](docs/UserClaimAccountRequest.md)
 - [UserEventType](docs/UserEventType.md)
 - [UserResetPasswordRequest](docs/UserResetPasswordRequest.md)
 - [UserStatus](docs/UserStatus.md)
 - [UserWithAppMetadata](docs/UserWithAppMetadata.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


