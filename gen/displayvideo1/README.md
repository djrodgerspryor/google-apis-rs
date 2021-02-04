<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/api/README.md.mako'
DO NOT EDIT !
-->
The `google-displayvideo1` library allows access to all features of the *Google Display Video* service.

This documentation was generated from *Display Video* crate version *1.0.14+20200707*, where *20200707* is the exact revision of the *displayvideo:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.

Everything else about the *Display Video* *v1* API can be found at the
[official documentation site](https://developers.google.com/display-video/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/DisplayVideo) ... 

* [advertisers](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::Advertiser)
 * [*assets upload*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserAssetUploadCall), [*bulk edit advertiser assigned targeting options*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserBulkEditAdvertiserAssignedTargetingOptionCall), [*bulk list advertiser assigned targeting options*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserBulkListAdvertiserAssignedTargetingOptionCall), [*campaigns create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCampaignCreateCall), [*campaigns delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCampaignDeleteCall), [*campaigns get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCampaignGetCall), [*campaigns list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCampaignListCall), [*campaigns patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCampaignPatchCall), [*channels create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelCreateCall), [*channels get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelGetCall), [*channels list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelListCall), [*channels patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelPatchCall), [*channels sites bulk edit*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelSiteBulkEditCall), [*channels sites create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelSiteCreateCall), [*channels sites delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelSiteDeleteCall), [*channels sites list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserChannelSiteListCall), [*create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCreateCall), [*creatives create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCreativeCreateCall), [*creatives delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCreativeDeleteCall), [*creatives get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCreativeGetCall), [*creatives list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCreativeListCall), [*creatives patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserCreativePatchCall), [*delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserDeleteCall), [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserGetCall), [*insertion orders create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserInsertionOrderCreateCall), [*insertion orders delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserInsertionOrderDeleteCall), [*insertion orders get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserInsertionOrderGetCall), [*insertion orders list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserInsertionOrderListCall), [*insertion orders patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserInsertionOrderPatchCall), [*line items bulk edit line item assigned targeting options*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemBulkEditLineItemAssignedTargetingOptionCall), [*line items bulk list line item assigned targeting options*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemBulkListLineItemAssignedTargetingOptionCall), [*line items create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemCreateCall), [*line items delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemDeleteCall), [*line items get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemGetCall), [*line items list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemListCall), [*line items patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemPatchCall), [*line items targeting types assigned targeting options create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionCreateCall), [*line items targeting types assigned targeting options delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionDeleteCall), [*line items targeting types assigned targeting options get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionGetCall), [*line items targeting types assigned targeting options list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLineItemTargetingTypeAssignedTargetingOptionListCall), [*list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserListCall), [*location lists assigned locations bulk edit*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListAssignedLocationBulkEditCall), [*location lists assigned locations create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListAssignedLocationCreateCall), [*location lists assigned locations delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListAssignedLocationDeleteCall), [*location lists assigned locations list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListAssignedLocationListCall), [*location lists create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListCreateCall), [*location lists get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListGetCall), [*location lists list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListListCall), [*location lists patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserLocationListPatchCall), [*negative keyword lists create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListCreateCall), [*negative keyword lists delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListDeleteCall), [*negative keyword lists get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListGetCall), [*negative keyword lists list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListListCall), [*negative keyword lists negative keywords bulk edit*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordBulkEditCall), [*negative keyword lists negative keywords create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordCreateCall), [*negative keyword lists negative keywords delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordDeleteCall), [*negative keyword lists negative keywords list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListNegativeKeywordListCall), [*negative keyword lists patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserNegativeKeywordListPatchCall), [*patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserPatchCall), [*targeting types assigned targeting options create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionCreateCall), [*targeting types assigned targeting options delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionDeleteCall), [*targeting types assigned targeting options get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionGetCall) and [*targeting types assigned targeting options list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserTargetingTypeAssignedTargetingOptionListCall)
* [combined audiences](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::CombinedAudience)
 * [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::CombinedAudienceGetCall) and [*list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::CombinedAudienceListCall)
* [custom lists](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::CustomList)
 * [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::CustomListGetCall) and [*list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::CustomListListCall)
* [first and third party audiences](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::FirstAndThirdPartyAudience)
 * [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::FirstAndThirdPartyAudienceGetCall) and [*list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::FirstAndThirdPartyAudienceListCall)
* [floodlight groups](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::FloodlightGroup)
 * [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::FloodlightGroupGetCall) and [*patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::FloodlightGroupPatchCall)
* [google audiences](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::GoogleAudience)
 * [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::GoogleAudienceGetCall) and [*list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::GoogleAudienceListCall)
* [inventory source groups](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroup)
 * [*assigned inventory sources bulk edit*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceBulkEditCall), [*assigned inventory sources create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceCreateCall), [*assigned inventory sources delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceDeleteCall), [*assigned inventory sources list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupAssignedInventorySourceListCall), [*create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupCreateCall), [*delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupDeleteCall), [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupGetCall), [*list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupListCall) and [*patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGroupPatchCall)
* [inventory sources](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySource)
 * [*get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceGetCall) and [*list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::InventorySourceListCall)
* media
 * [*download*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::MediaDownloadCall)
* partners
 * [*channels create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelCreateCall), [*channels get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelGetCall), [*channels list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelListCall), [*channels patch*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelPatchCall), [*channels sites bulk edit*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelSiteBulkEditCall), [*channels sites create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelSiteCreateCall), [*channels sites delete*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelSiteDeleteCall) and [*channels sites list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::PartnerChannelSiteListCall)
* sdfdownloadtasks
 * [*create*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::SdfdownloadtaskCreateCall) and [*operations get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::SdfdownloadtaskOperationGetCall)
* targeting types
 * [*targeting options get*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::TargetingTypeTargetingOptionGetCall) and [*targeting options list*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::TargetingTypeTargetingOptionListCall)


Upload supported by ...

* [*assets upload advertisers*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::AdvertiserAssetUploadCall)

Download supported by ...

* [*download media*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/api::MediaDownloadCall)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/DisplayVideo)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::CallBuilder)
* **[Resources](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit()
```

Or specifically ...

```ignore
let r = hub.advertisers().assets_upload(...).doit()
let r = hub.advertisers().campaigns_create(...).doit()
let r = hub.advertisers().campaigns_delete(...).doit()
let r = hub.advertisers().campaigns_get(...).doit()
let r = hub.advertisers().campaigns_list(...).doit()
let r = hub.advertisers().campaigns_patch(...).doit()
let r = hub.advertisers().channels_sites_bulk_edit(...).doit()
let r = hub.advertisers().channels_sites_create(...).doit()
let r = hub.advertisers().channels_sites_delete(...).doit()
let r = hub.advertisers().channels_sites_list(...).doit()
let r = hub.advertisers().channels_create(...).doit()
let r = hub.advertisers().channels_get(...).doit()
let r = hub.advertisers().channels_list(...).doit()
let r = hub.advertisers().channels_patch(...).doit()
let r = hub.advertisers().creatives_create(...).doit()
let r = hub.advertisers().creatives_delete(...).doit()
let r = hub.advertisers().creatives_get(...).doit()
let r = hub.advertisers().creatives_list(...).doit()
let r = hub.advertisers().creatives_patch(...).doit()
let r = hub.advertisers().insertion_orders_create(...).doit()
let r = hub.advertisers().insertion_orders_delete(...).doit()
let r = hub.advertisers().insertion_orders_get(...).doit()
let r = hub.advertisers().insertion_orders_list(...).doit()
let r = hub.advertisers().insertion_orders_patch(...).doit()
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_create(...).doit()
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_delete(...).doit()
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_get(...).doit()
let r = hub.advertisers().line_items_targeting_types_assigned_targeting_options_list(...).doit()
let r = hub.advertisers().line_items_bulk_edit_line_item_assigned_targeting_options(...).doit()
let r = hub.advertisers().line_items_bulk_list_line_item_assigned_targeting_options(...).doit()
let r = hub.advertisers().line_items_create(...).doit()
let r = hub.advertisers().line_items_delete(...).doit()
let r = hub.advertisers().line_items_get(...).doit()
let r = hub.advertisers().line_items_list(...).doit()
let r = hub.advertisers().line_items_patch(...).doit()
let r = hub.advertisers().location_lists_assigned_locations_bulk_edit(...).doit()
let r = hub.advertisers().location_lists_assigned_locations_create(...).doit()
let r = hub.advertisers().location_lists_assigned_locations_delete(...).doit()
let r = hub.advertisers().location_lists_assigned_locations_list(...).doit()
let r = hub.advertisers().location_lists_create(...).doit()
let r = hub.advertisers().location_lists_get(...).doit()
let r = hub.advertisers().location_lists_list(...).doit()
let r = hub.advertisers().location_lists_patch(...).doit()
let r = hub.advertisers().negative_keyword_lists_negative_keywords_bulk_edit(...).doit()
let r = hub.advertisers().negative_keyword_lists_negative_keywords_create(...).doit()
let r = hub.advertisers().negative_keyword_lists_negative_keywords_delete(...).doit()
let r = hub.advertisers().negative_keyword_lists_negative_keywords_list(...).doit()
let r = hub.advertisers().negative_keyword_lists_create(...).doit()
let r = hub.advertisers().negative_keyword_lists_delete(...).doit()
let r = hub.advertisers().negative_keyword_lists_get(...).doit()
let r = hub.advertisers().negative_keyword_lists_list(...).doit()
let r = hub.advertisers().negative_keyword_lists_patch(...).doit()
let r = hub.advertisers().targeting_types_assigned_targeting_options_create(...).doit()
let r = hub.advertisers().targeting_types_assigned_targeting_options_delete(...).doit()
let r = hub.advertisers().targeting_types_assigned_targeting_options_get(...).doit()
let r = hub.advertisers().targeting_types_assigned_targeting_options_list(...).doit()
let r = hub.advertisers().bulk_edit_advertiser_assigned_targeting_options(...).doit()
let r = hub.advertisers().bulk_list_advertiser_assigned_targeting_options(...).doit()
let r = hub.advertisers().create(...).doit()
let r = hub.advertisers().delete(...).doit()
let r = hub.advertisers().get(...).doit()
let r = hub.advertisers().list(...).doit()
let r = hub.advertisers().patch(...).doit()
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-displayvideo1 = "*"
# This project intentionally uses an old version of Hyper. See
# https://github.com/Byron/google-apis-rs/issues/173 for more
# information.
hyper = "^0.10"
hyper-rustls = "^0.6"
serde = "^1.0"
serde_json = "^1.0"
yup-oauth2 = "^1.0"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_displayvideo1 as displayvideo1;
use displayvideo1::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use displayvideo1::DisplayVideo;

// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                              <MemoryStorage as Default>::default(), None);
let mut hub = DisplayVideo::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.advertisers().channels_sites_list("advertiserId", "channelId")
             .partner_id("no")
             .page_token("ipsum")
             .page_size(-28)
             .order_by("At")
             .filter("sanctus")
             .doit();

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the 
[Hub Delegate](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols: 
*simple* and *resumable*. The distinctiveness of each is represented by customized 
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Delegate) to the 
[Method Builder](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::CallBuilder) before making the final `doit()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The [delegate trait](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::RequestValue) and 
[decodable](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses 
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::Part) which are identifiable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-displayvideo1/1.0.14+20200707/google_displayvideo1/client::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

# License
The **displayvideo1** library was generated by Sebastian Thiel, and is placed 
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/master/LICENSE.md
