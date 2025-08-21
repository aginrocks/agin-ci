# \SystemApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_runner**](SystemApi.md#create_runner) | **POST** /api/system/runners | Create runner
[**delete_runner**](SystemApi.md#delete_runner) | **DELETE** /api/system/runners/{runner_id} | Delete runner
[**edit_runner**](SystemApi.md#edit_runner) | **PATCH** /api/system/runners/{runner_id} | Edit runner
[**edit_system_user**](SystemApi.md#edit_system_user) | **PATCH** /api/system/users/{user_id} | Edit user's role
[**finish_runner_registration**](SystemApi.md#finish_runner_registration) | **POST** /api/system/runners/register/finish | Finish runner registration
[**get_runners**](SystemApi.md#get_runners) | **GET** /api/system/runners | Get runners
[**get_system_users**](SystemApi.md#get_system_users) | **GET** /api/system/users | Get all users



## create_runner

> models::RegisterRunnerResponse create_runner(register_runner_body)
Create runner

Creates a new runner in the system. Returns a token that the runner can use to register itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_runner_body** | [**RegisterRunnerBody**](RegisterRunnerBody.md) |  | [required] |

### Return type

[**models::RegisterRunnerResponse**](RegisterRunnerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_runner

> delete_runner(runner_id)
Delete runner

Permanently delete a runner without deleting the job runs associated with it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **String** | Runner ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_runner

> models::CreateSuccess edit_runner(runner_id, register_runner_body)
Edit runner

Edit runner's details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**runner_id** | **String** | Runner ID | [required] |
**register_runner_body** | [**RegisterRunnerBody**](RegisterRunnerBody.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_system_user

> Vec<models::User> edit_system_user(user_id, edit_server_role_body)
Edit user's role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**edit_server_role_body** | [**EditServerRoleBody**](EditServerRoleBody.md) |  | [required] |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## finish_runner_registration

> models::FinishRegistrationResponse finish_runner_registration(finish_registration_body)
Finish runner registration

This endpoint allows to exchange the registration token for a long-lived access token. The token can be used to authenticate directly to Apache Pulsar.  No normal authentication is required, but the registration token must be valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**finish_registration_body** | [**FinishRegistrationBody**](FinishRegistrationBody.md) |  | [required] |

### Return type

[**models::FinishRegistrationResponse**](FinishRegistrationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_runners

> Vec<models::Runner> get_runners()
Get runners

This endpoint returns all runners that are registered in the system.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Runner>**](Runner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_users

> Vec<models::User> get_system_users()
Get all users

Returns every user in the system.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::User>**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

