# \TokensApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_token**](TokensApi.md#create_token) | **POST** /api/tokens | Create token
[**get_tokens**](TokensApi.md#get_tokens) | **GET** /api/tokens | Get tokens



## create_token

> models::AccessTokenCreateResponse create_token(access_token_create_body)
Create token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token_create_body** | [**AccessTokenCreateBody**](AccessTokenCreateBody.md) |  | [required] |

### Return type

[**models::AccessTokenCreateResponse**](AccessTokenCreateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tokens

> models::User get_tokens()
Get tokens

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

