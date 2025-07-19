# \ProjectsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectsApi.md#create_project) | **POST** /api/organizations/{org_slug}/projects | Create project
[**get_projects**](ProjectsApi.md#get_projects) | **GET** /api/organizations/{org_slug}/projects | Get projects



## create_project

> models::CreateSuccess create_project(org_slug, create_project_body)
Create project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**create_project_body** | [**CreateProjectBody**](CreateProjectBody.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects

> Vec<models::PublicProject> get_projects(org_slug)
Get projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |

### Return type

[**Vec<models::PublicProject>**](PublicProject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

