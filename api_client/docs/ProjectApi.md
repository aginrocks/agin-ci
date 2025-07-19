# \ProjectApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_project**](ProjectApi.md#delete_project) | **DELETE** /api/organizations/{org_slug}/projects/{project_slug} | Delete project
[**edit_project**](ProjectApi.md#edit_project) | **PATCH** /api/organizations/{org_slug}/projects/{project_slug} | Edit project
[**get_project**](ProjectApi.md#get_project) | **GET** /api/organizations/{org_slug}/projects/{project_slug} | Get project
[**regenerate_project_keys**](ProjectApi.md#regenerate_project_keys) | **GET** /api/organizations/{org_slug}/projects/{project_slug}/regenerate-keys | Regenerate project deploy keys
[**regenerate_webhook_secret**](ProjectApi.md#regenerate_webhook_secret) | **GET** /api/organizations/{org_slug}/projects/{project_slug}/regenerate-webhook-secret | Regenerate webhook secret



## delete_project

> delete_project(org_slug, project_slug)
Delete project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**project_slug** | **String** | Project slug | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_project

> models::CreateSuccess edit_project(org_slug, project_slug, create_project_body)
Edit project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**project_slug** | **String** | Project slug | [required] |
**create_project_body** | [**CreateProjectBody**](CreateProjectBody.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> models::PublicProject get_project(org_slug, project_slug)
Get project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**project_slug** | **String** | Project slug | [required] |

### Return type

[**models::PublicProject**](PublicProject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_project_keys

> models::PublicProject regenerate_project_keys(org_slug, project_slug)
Regenerate project deploy keys

These keys are used to pull the repository. You can get the public key from the project details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**project_slug** | **String** | Project slug | [required] |

### Return type

[**models::PublicProject**](PublicProject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_webhook_secret

> models::RegenerateSecretResponse regenerate_webhook_secret(org_slug, project_slug)
Regenerate webhook secret

This secret is used to verify the authenticity of webhooks sent by the repository service. You won't be able to view it again after this call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**project_slug** | **String** | Project slug | [required] |

### Return type

[**models::RegenerateSecretResponse**](RegenerateSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

