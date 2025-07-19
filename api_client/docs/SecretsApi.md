# \SecretsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_secret**](SecretsApi.md#create_organization_secret) | **POST** /api/organizations/{org_slug}/secrets | Create org secret
[**delete_organization_secret**](SecretsApi.md#delete_organization_secret) | **DELETE** /api/organizations/{org_slug}/secrets/{secret_id} | Delete org secret
[**edit_organization_secret**](SecretsApi.md#edit_organization_secret) | **PATCH** /api/organizations/{org_slug}/secrets/{secret_id} | Edit org secret
[**get_organization_secrets**](SecretsApi.md#get_organization_secrets) | **GET** /api/organizations/{org_slug}/secrets | Get org secrets



## create_organization_secret

> models::CreateSuccess create_organization_secret(org_slug, create_org_secret_body)
Create org secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**create_org_secret_body** | [**CreateOrgSecretBody**](CreateOrgSecretBody.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_secret

> delete_organization_secret(org_slug, secret_id)
Delete org secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**secret_id** | **String** | Secret ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_secret

> models::CreateSuccess edit_organization_secret(org_slug, secret_id, edit_org_secret_body)
Edit org secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**secret_id** | **String** | Secret ID | [required] |
**edit_org_secret_body** | [**EditOrgSecretBody**](EditOrgSecretBody.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_secrets

> Vec<models::PublicSecret> get_organization_secrets(org_slug)
Get org secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |

### Return type

[**Vec<models::PublicSecret>**](PublicSecret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

