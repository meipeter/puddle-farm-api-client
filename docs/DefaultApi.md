# \DefaultApi

All URIs are relative to *https://puddle.farm/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alias_player_id_get**](DefaultApi.md#alias_player_id_get) | **GET** /alias/{player_id} | Get player's aliases
[**avatar_player_id_get**](DefaultApi.md#avatar_player_id_get) | **GET** /avatar/{player_id} | Get player's avatar image
[**calc_rating_get**](DefaultApi.md#calc_rating_get) | **GET** /calc_rating | Calculate rating changes for a match
[**characters_get**](DefaultApi.md#characters_get) | **GET** /characters | Get a list of all characters
[**claim_player_id_get**](DefaultApi.md#claim_player_id_get) | **GET** /claim/{player_id} | Initiate a claim for a player's profile
[**claim_poll_player_id_get**](DefaultApi.md#claim_poll_player_id_get) | **GET** /claim/poll/{player_id} | Poll for the status of a player's profile claim
[**distribution_get**](DefaultApi.md#distribution_get) | **GET** /distribution | Get player rating distribution data
[**health_get**](DefaultApi.md#health_get) | **GET** /health | Get health status of the system
[**matchups_get**](DefaultApi.md#matchups_get) | **GET** /matchups | Get character matchup data
[**matchups_player_id_char_id_duration_get**](DefaultApi.md#matchups_player_id_char_id_duration_get) | **GET** /matchups/{player_id}/{char_id}/{duration} | Get player's character matchup data
[**player_id_get**](DefaultApi.md#player_id_get) | **GET** /player/{id} | Get player by ID
[**player_player_id_char_id_history_get**](DefaultApi.md#player_player_id_char_id_history_get) | **GET** /player/{player_id}/{char_id}/history | Get player's match history for a specific character
[**player_search_get**](DefaultApi.md#player_search_get) | **GET** /player/search | Search for players by name
[**popularity_get**](DefaultApi.md#popularity_get) | **GET** /popularity | Get character popularity data
[**ratings_player_id_char_id_duration_get**](DefaultApi.md#ratings_player_id_char_id_duration_get) | **GET** /ratings/{player_id}/{char_id}/{duration} | Get player's rating history for a specific character
[**settings_key_get**](DefaultApi.md#settings_key_get) | **GET** /settings/{key} | Get player's settings
[**stats_get**](DefaultApi.md#stats_get) | **GET** /stats | Get global statistics
[**supporters_get**](DefaultApi.md#supporters_get) | **GET** /supporters | Get list of supporters
[**toggle_private_key_get**](DefaultApi.md#toggle_private_key_get) | **GET** /toggle_private/{key} | Toggle player's privacy setting
[**top_char_char_id_get**](DefaultApi.md#top_char_char_id_get) | **GET** /top_char/{char_id} | Get top ranked players for a specific character
[**top_get**](DefaultApi.md#top_get) | **GET** /top | Get top ranked players



## alias_player_id_get

> Vec<String> alias_player_id_get(player_id)
Get player's aliases

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** | ID of the player | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatar_player_id_get

> std::path::PathBuf avatar_player_id_get(player_id)
Get player's avatar image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** | ID of the player | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calc_rating_get

> models::CalcRatingResponse calc_rating_get(rating_a, drift_a, rating_b, drift_b, a_wins)
Calculate rating changes for a match

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rating_a** | **f32** | Player A's current rating | [required] |
**drift_a** | **f32** | Player A's current drift (must be > 1.0) | [required] |
**rating_b** | **f32** | Player B's current rating | [required] |
**drift_b** | **f32** | Player B's current drift (must be > 1.0) | [required] |
**a_wins** | **bool** | Whether player A wins the match | [required] |

### Return type

[**models::CalcRatingResponse**](CalcRatingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## characters_get

> Vec<Vec<String>> characters_get()
Get a list of all characters

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<Vec<String>>**](Vec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## claim_player_id_get

> String claim_player_id_get(player_id)
Initiate a claim for a player's profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** | ID of the player | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## claim_poll_player_id_get

> String claim_poll_player_id_get(player_id)
Poll for the status of a player's profile claim

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** | ID of the player | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## distribution_get

> models::DistributionResponse distribution_get()
Get player rating distribution data

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DistributionResponse**](DistributionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get

> String health_get()
Get health status of the system

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## matchups_get

> models::MatchupResponse matchups_get()
Get character matchup data

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MatchupResponse**](MatchupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## matchups_player_id_char_id_duration_get

> models::MatchupCharResponse matchups_player_id_char_id_duration_get(player_id, char_id, duration)
Get player's character matchup data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** | ID of the player | [required] |
**char_id** | **String** | Short name of the character (e.g., \"SO\" for Sol) | [required] |
**duration** | **i32** | Duration in days for the matchup data | [required] |

### Return type

[**models::MatchupCharResponse**](MatchupCharResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_id_get

> models::PlayerResponse player_id_get(id)
Get player by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of the player to get | [required] |

### Return type

[**models::PlayerResponse**](PlayerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_player_id_char_id_history_get

> models::PlayerGamesResponse player_player_id_char_id_history_get(player_id, char_id, count, offset)
Get player's match history for a specific character

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** | ID of the player | [required] |
**char_id** | **String** | Short name of the character (e.g., \"SO\" for Sol) | [required] |
**count** | Option<**i32**> | Number of matches to return (default 100) |  |[default to 100]
**offset** | Option<**i32**> | Number of matches to skip (default 0) |  |[default to 0]

### Return type

[**models::PlayerGamesResponse**](PlayerGamesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_search_get

> models::SearchResponse player_search_get(search_string, exact)
Search for players by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_string** | **String** | The string to search for in player names | [required] |
**exact** | Option<**bool**> | Whether to perform an exact match (true) or a partial match (false) |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## popularity_get

> models::PopularityResult popularity_get()
Get character popularity data

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PopularityResult**](PopularityResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ratings_player_id_char_id_duration_get

> Vec<models::RatingsResponse> ratings_player_id_char_id_duration_get(player_id, char_id, duration)
Get player's rating history for a specific character

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i64** | ID of the player | [required] |
**char_id** | **String** | Short name of the character (e.g., \"SO\" for Sol) | [required] |
**duration** | **i32** | Duration in days for the rating history | [required] |

### Return type

[**Vec<models::RatingsResponse>**](RatingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_key_get

> models::SettingsResponse settings_key_get(key)
Get player's settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Player's API key | [required] |

### Return type

[**models::SettingsResponse**](SettingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stats_get

> models::StatsResponse stats_get()
Get global statistics

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StatsResponse**](StatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supporters_get

> Vec<models::Supporter> supporters_get()
Get list of supporters

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Supporter>**](Supporter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_private_key_get

> String toggle_private_key_get(key)
Toggle player's privacy setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Player's API key | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## top_char_char_id_get

> models::RankResponse top_char_char_id_get(char_id, count, offset)
Get top ranked players for a specific character

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**char_id** | **String** | Short name of the character (e.g., \"SO\" for Sol) | [required] |
**count** | Option<**i32**> | Number of players to return (default 100) |  |[default to 100]
**offset** | Option<**i32**> | Number of players to skip (default 0) |  |[default to 0]

### Return type

[**models::RankResponse**](RankResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## top_get

> models::RankResponse top_get(count, offset)
Get top ranked players

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of players to return (default 100) |  |[default to 100]
**offset** | Option<**i32**> | Number of players to skip (default 0) |  |[default to 0]

### Return type

[**models::RankResponse**](RankResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

