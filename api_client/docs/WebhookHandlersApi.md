# \WebhookHandlersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gitea_webhook_handler**](WebhookHandlersApi.md#gitea_webhook_handler) | **POST** /api/webhooks/{project_id}/gitea | Forgejo or Gitea Webhook handler
[**github_webhook_handler**](WebhookHandlersApi.md#github_webhook_handler) | **POST** /api/webhooks/{project_id}/github | GitHub Webhook handler



## gitea_webhook_handler

> models::WebhookHandlerSuccess gitea_webhook_handler(project_id, body)
Forgejo or Gitea Webhook handler

Handles incoming Forgejo or Gitea webhooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project ID | [required] |
**body** | **String** |  | [required] |

### Return type

[**models::WebhookHandlerSuccess**](WebhookHandlerSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## github_webhook_handler

> models::WebhookHandlerSuccess github_webhook_handler(project_id, body)
GitHub Webhook handler

Handles incoming GitHub webhooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | Project ID | [required] |
**body** | **String** |  | [required] |

### Return type

[**models::WebhookHandlerSuccess**](WebhookHandlerSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

