# \GodModeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_god_mode**](GodModeApi.md#change_god_mode) | **PATCH** /api/god | Change God Mode status
[**get_god_mode**](GodModeApi.md#get_god_mode) | **GET** /api/god | Get God Mode status



## change_god_mode

> models::GodModeStatus change_god_mode(god_mode_body)
Change God Mode status

Enable or disable God Mode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**god_mode_body** | [**GodModeBody**](GodModeBody.md) |  | [required] |

### Return type

[**models::GodModeStatus**](GodModeStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_god_mode

> models::GodModeStatus get_god_mode()
Get God Mode status

God Mode is a special mode that allows the user to bypass every permission check. It can only be anabled by system admins.  This endpoint won't return a 403 Forbidden error even if you don't have the required permissions to enable God Mode.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GodModeStatus**](GodModeStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

