# \NotificationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_notification_status**](NotificationsApi.md#edit_notification_status) | **PATCH** /api/notifications/{notification_id} | Edit notification status
[**get_notification**](NotificationsApi.md#get_notification) | **GET** /api/notifications/{notification_id} | Get notification
[**get_notifications**](NotificationsApi.md#get_notifications) | **GET** /api/notifications | Get notifications



## edit_notification_status

> models::CreateSuccess edit_notification_status(notification_id, edit_notification_body)
Edit notification status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Notification ID | [required] |
**edit_notification_body** | [**EditNotificationBody**](EditNotificationBody.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification

> models::NotificationDetailed get_notification(notification_id)
Get notification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Notification ID | [required] |

### Return type

[**models::NotificationDetailed**](Notification_Detailed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> Vec<models::NotificationDetailed> get_notifications()
Get notifications

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NotificationDetailed>**](Notification_Detailed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

