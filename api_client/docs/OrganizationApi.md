# \OrganizationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organization_member**](OrganizationApi.md#add_organization_member) | **PUT** /api/organizations/{org_slug}/members | Add org member
[**delete_organization**](OrganizationApi.md#delete_organization) | **DELETE** /api/organizations/{org_slug} | Delete org
[**delete_organization_member**](OrganizationApi.md#delete_organization_member) | **DELETE** /api/organizations/{org_slug}/members/{member_id} | Delete org member
[**edit_organization**](OrganizationApi.md#edit_organization) | **PATCH** /api/organizations/{org_slug} | Edit org
[**edit_organization_member**](OrganizationApi.md#edit_organization_member) | **PATCH** /api/organizations/{org_slug}/members/{member_id} | Edit org member's role
[**get_organization**](OrganizationApi.md#get_organization) | **GET** /api/organizations/{org_slug} | Get org
[**get_organization_members**](OrganizationApi.md#get_organization_members) | **GET** /api/organizations/{org_slug}/members | Get org members



## add_organization_member

> models::CreateSuccess add_organization_member(org_slug, membership)
Add org member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**membership** | [**Membership**](Membership.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> delete_organization(org_slug)
Delete org

Dangerous!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_member

> delete_organization_member(org_slug, member_id)
Delete org member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**member_id** | **String** | Member ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization

> models::CreateSuccess edit_organization(org_slug, mutable_organization)
Edit org

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**mutable_organization** | [**MutableOrganization**](MutableOrganization.md) |  | [required] |

### Return type

[**models::CreateSuccess**](CreateSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_member

> edit_organization_member(org_slug, member_id, edit_role_body)
Edit org member's role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |
**member_id** | **String** | Member ID | [required] |
**edit_role_body** | [**EditRoleBody**](EditRoleBody.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::Organization get_organization(org_slug)
Get org

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_members

> Vec<models::Member> get_organization_members(org_slug)
Get org members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Organization slug | [required] |

### Return type

[**Vec<models::Member>**](Member.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

