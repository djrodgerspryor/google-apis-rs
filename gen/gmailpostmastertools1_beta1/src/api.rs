use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all PostmasterTools related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gmailpostmastertools1_beta1 as gmailpostmastertools1_beta1;
/// use gmailpostmastertools1_beta1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2;
/// use gmailpostmastertools1_beta1::PostmasterTools;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PostmasterTools::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.domains().traffic_stats_list("parent")
///              .start_date_year(-55)
///              .start_date_month(-88)
///              .start_date_day(-47)
///              .page_token("duo")
///              .page_size(-50)
///              .end_date_year(-93)
///              .end_date_month(-37)
///              .end_date_day(-12)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct PostmasterTools<C> {
    client: RefCell<C>,
    auth: RefCell<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C> client::Hub for PostmasterTools<C> {}

impl<'a, C> PostmasterTools<C>
    where  C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {

    pub fn new(client: C, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> PostmasterTools<C> {
        PostmasterTools {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.14".to_string(),
            _base_url: "https://gmailpostmastertools.googleapis.com/".to_string(),
            _root_url: "https://gmailpostmastertools.googleapis.com/".to_string(),
        }
    }

    pub fn domains(&'a self) -> DomainMethods<'a, C> {
        DomainMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.14`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://gmailpostmastertools.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://gmailpostmastertools.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Metric on a particular delivery error type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryError {
    /// The class of delivery error.
    #[serde(rename="errorClass")]
    pub error_class: Option<String>,
    /// The ratio of messages where the error occurred vs all authenticated
    /// traffic.
    #[serde(rename="errorRatio")]
    pub error_ratio: Option<f64>,
    /// The type of delivery error.
    #[serde(rename="errorType")]
    pub error_type: Option<String>,
}

impl client::Part for DeliveryError {}


/// A registered domain resource in the Postmaster API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traffic stats get domains](DomainTrafficStatGetCall) (none)
/// * [traffic stats list domains](DomainTrafficStatListCall) (none)
/// * [get domains](DomainGetCall) (response)
/// * [list domains](DomainListCall) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Domain {
    /// Timestamp when the user registered this domain. Assigned by
    /// the server.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// The resource name of the Domain.
    /// Domain names have the form `domains/{domain_name}`, where domain_name
    /// is the fully qualified domain name (i.e.,  mymail.mydomain.com).
    pub name: Option<String>,
    /// User’s permission for this domain. Assigned by the server.
    pub permission: Option<String>,
}

impl client::Resource for Domain {}
impl client::ResponseResult for Domain {}


/// [Feedback loop](https://support.google.com/mail/answer/6254652) identifier
/// information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackLoop {
    /// Feedback loop identifier that uniquely identifies individual campaigns.
    pub id: Option<String>,
    /// The ratio of user marked spam messages with the identifier vs the total
    /// number of inboxed messages with that identifier.
    #[serde(rename="spamRatio")]
    pub spam_ratio: Option<f64>,
}

impl client::Part for FeedbackLoop {}


/// IP Reputation information for a set of IPs in a specific reputation category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IpReputation {
    /// Total number of unique IPs in this reputation category. This metric only
    /// pertains to traffic that passed [SPF](http://www.openspf.org/) or
    /// [DKIM](http://www.dkim.org/).
    #[serde(rename="numIps")]
    pub num_ips: Option<String>,
    /// The reputation category this IP reputation represents.
    pub reputation: Option<String>,
    /// A sample of IPs in this reputation category.
    #[serde(rename="sampleIps")]
    pub sample_ips: Option<Vec<String>>,
}

impl client::Part for IpReputation {}


/// Response message for ListDomains.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list domains](DomainListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDomainsResponse {
    /// The list of domains.
    pub domains: Option<Vec<Domain>>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDomainsResponse {}


/// Response message for ListTrafficStats.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traffic stats list domains](DomainTrafficStatListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTrafficStatsResponse {
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of TrafficStats.
    #[serde(rename="trafficStats")]
    pub traffic_stats: Option<Vec<TrafficStats>>,
}

impl client::ResponseResult for ListTrafficStatsResponse {}


/// Email traffic statistics pertaining to a specific date.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [traffic stats get domains](DomainTrafficStatGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrafficStats {
    /// Delivery errors for the domain. This metric only pertains to traffic that
    /// passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/).
    #[serde(rename="deliveryErrors")]
    pub delivery_errors: Option<Vec<DeliveryError>>,
    /// The ratio of mail that successfully authenticated with DKIM vs. all mail
    /// that attempted to authenticate with [DKIM](http://www.dkim.org/). Spoofed
    /// mail is excluded.
    #[serde(rename="dkimSuccessRatio")]
    pub dkim_success_ratio: Option<f64>,
    /// The ratio of mail that passed [DMARC](https://dmarc.org/) alignment checks
    /// vs all mail received from the domain that successfully authenticated with
    /// either of [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/).
    #[serde(rename="dmarcSuccessRatio")]
    pub dmarc_success_ratio: Option<f64>,
    /// Reputation of the domain.
    #[serde(rename="domainReputation")]
    pub domain_reputation: Option<String>,
    /// The ratio of incoming mail (to Gmail), that passed secure transport (TLS)
    /// vs all mail received from that domain. This metric only pertains to traffic
    /// that passed [SPF](http://www.openspf.org/) or [DKIM](http://www.dkim.org/).
    #[serde(rename="inboundEncryptionRatio")]
    pub inbound_encryption_ratio: Option<f64>,
    /// Reputation information pertaining to the IP addresses of the email servers
    /// for the domain. There is exactly one entry for each reputation category
    /// except
    /// REPUTATION_CATEGORY_UNSPECIFIED.
    #[serde(rename="ipReputations")]
    pub ip_reputations: Option<Vec<IpReputation>>,
    /// The resource name of the traffic statistics. Traffic statistic names have
    /// the form `domains/{domain}/trafficStats/{date}`, where
    /// domain_name is the fully qualified domain name (i.e.,  mymail.mydomain.com)
    /// of the domain this traffic statistics pertains to and date is the date in
    /// yyyymmdd format that these statistics corresponds to.
    /// For example: domains/mymail.mydomain.com/trafficStats/20160807
    pub name: Option<String>,
    /// The ratio of outgoing mail (from Gmail) that was accepted over secure
    /// transport (TLS).
    #[serde(rename="outboundEncryptionRatio")]
    pub outbound_encryption_ratio: Option<f64>,
    /// Spammy [Feedback loop identifiers]
    /// (https://support.google.com/mail/answer/6254652) with their individual spam
    /// rates. This metric only pertains to traffic that is authenticated by
    /// [DKIM](http://www.dkim.org/).
    #[serde(rename="spammyFeedbackLoops")]
    pub spammy_feedback_loops: Option<Vec<FeedbackLoop>>,
    /// The ratio of mail that successfully authenticated with SPF vs. all mail
    /// that attempted to authenticate with [SPF](http://www.openspf.org/). Spoofed
    /// mail is excluded.
    #[serde(rename="spfSuccessRatio")]
    pub spf_success_ratio: Option<f64>,
    /// The ratio of user-report spam vs. email that was sent to the inbox. This
    /// metric only pertains to emails authenticated by
    /// [DKIM](http://www.dkim.org/).
    #[serde(rename="userReportedSpamRatio")]
    pub user_reported_spam_ratio: Option<f64>,
}

impl client::ResponseResult for TrafficStats {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *domain* resources.
/// It is not used directly, but through the `PostmasterTools` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_gmailpostmastertools1_beta1 as gmailpostmastertools1_beta1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2;
/// use gmailpostmastertools1_beta1::PostmasterTools;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PostmasterTools::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `traffic_stats_get(...)` and `traffic_stats_list(...)`
/// // to build up your call.
/// let rb = hub.domains();
/// # }
/// ```
pub struct DomainMethods<'a, C>
    where C: 'a {

    hub: &'a PostmasterTools<C>,
}

impl<'a, C> client::MethodsBuilder for DomainMethods<'a, C> {}

impl<'a, C> DomainMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get traffic statistics for a domain on a specific date.
    /// Returns PERMISSION_DENIED if user does not have permission to access
    /// TrafficStats for the domain.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the traffic statistics to get.
    ///            E.g., domains/mymail.mydomain.com/trafficStats/20160807.
    pub fn traffic_stats_get(&self, name: &str) -> DomainTrafficStatGetCall<'a, C> {
        DomainTrafficStatGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List traffic statistics for all available days.
    /// Returns PERMISSION_DENIED if user does not have permission to access
    /// TrafficStats for the domain.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the domain whose traffic statistics we'd like to list.
    ///              It should have the form `domains/{domain_name}`, where domain_name is
    ///              the fully qualified domain name.
    pub fn traffic_stats_list(&self, parent: &str) -> DomainTrafficStatListCall<'a, C> {
        DomainTrafficStatListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific domain registered by the client.
    /// Returns NOT_FOUND if the domain does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the domain. It should have the form
    ///            `domains/{domain_name}`, where domain_name is the fully qualified
    ///            domain name.
    pub fn get(&self, name: &str) -> DomainGetCall<'a, C> {
        DomainGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the domains that have been registered by the client. The order of
    /// domains in the response is unspecified and non-deterministic. Newly created
    /// domains will not necessarily be added to the end of this list.
    pub fn list(&self) -> DomainListCall<'a, C> {
        DomainListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Get traffic statistics for a domain on a specific date.
/// Returns PERMISSION_DENIED if user does not have permission to access
/// TrafficStats for the domain.
///
/// A builder for the *trafficStats.get* method supported by a *domain* resource.
/// It is not used directly, but through a `DomainMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gmailpostmastertools1_beta1 as gmailpostmastertools1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2;
/// # use gmailpostmastertools1_beta1::PostmasterTools;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PostmasterTools::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.domains().traffic_stats_get("name")
///              .doit();
/// # }
/// ```
pub struct DomainTrafficStatGetCall<'a, C>
    where C: 'a {

    hub: &'a PostmasterTools<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> client::CallBuilder for DomainTrafficStatGetCall<'a, C> {}

impl<'a, C> DomainTrafficStatGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TrafficStats)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "gmailpostmastertools.domains.trafficStats.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+name}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The resource name of the traffic statistics to get.
    /// E.g., domains/mymail.mydomain.com/trafficStats/20160807.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> DomainTrafficStatGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DomainTrafficStatGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> DomainTrafficStatGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// List traffic statistics for all available days.
/// Returns PERMISSION_DENIED if user does not have permission to access
/// TrafficStats for the domain.
///
/// A builder for the *trafficStats.list* method supported by a *domain* resource.
/// It is not used directly, but through a `DomainMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gmailpostmastertools1_beta1 as gmailpostmastertools1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2;
/// # use gmailpostmastertools1_beta1::PostmasterTools;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PostmasterTools::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.domains().traffic_stats_list("parent")
///              .start_date_year(-50)
///              .start_date_month(-50)
///              .start_date_day(-7)
///              .page_token("gubergren")
///              .page_size(-17)
///              .end_date_year(-99)
///              .end_date_month(-56)
///              .end_date_day(-25)
///              .doit();
/// # }
/// ```
pub struct DomainTrafficStatListCall<'a, C>
    where C: 'a {

    hub: &'a PostmasterTools<C>,
    _parent: String,
    _start_date_year: Option<i32>,
    _start_date_month: Option<i32>,
    _start_date_day: Option<i32>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _end_date_year: Option<i32>,
    _end_date_month: Option<i32>,
    _end_date_day: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> client::CallBuilder for DomainTrafficStatListCall<'a, C> {}

impl<'a, C> DomainTrafficStatListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListTrafficStatsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "gmailpostmastertools.domains.trafficStats.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._start_date_year {
            params.push(("startDate.year", value.to_string()));
        }
        if let Some(value) = self._start_date_month {
            params.push(("startDate.month", value.to_string()));
        }
        if let Some(value) = self._start_date_day {
            params.push(("startDate.day", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._end_date_year {
            params.push(("endDate.year", value.to_string()));
        }
        if let Some(value) = self._end_date_month {
            params.push(("endDate.month", value.to_string()));
        }
        if let Some(value) = self._end_date_day {
            params.push(("endDate.day", value.to_string()));
        }
        for &field in ["alt", "parent", "startDate.year", "startDate.month", "startDate.day", "pageToken", "pageSize", "endDate.year", "endDate.month", "endDate.day"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+parent}/trafficStats";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["parent"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The resource name of the domain whose traffic statistics we'd like to list.
    /// It should have the form `domains/{domain_name}`, where domain_name is
    /// the fully qualified domain name.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> DomainTrafficStatListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    ///
    /// Sets the *start date.year* query property to the given value.
    pub fn start_date_year(mut self, new_value: i32) -> DomainTrafficStatListCall<'a, C> {
        self._start_date_year = Some(new_value);
        self
    }
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    ///
    /// Sets the *start date.month* query property to the given value.
    pub fn start_date_month(mut self, new_value: i32) -> DomainTrafficStatListCall<'a, C> {
        self._start_date_month = Some(new_value);
        self
    }
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    ///
    /// Sets the *start date.day* query property to the given value.
    pub fn start_date_day(mut self, new_value: i32) -> DomainTrafficStatListCall<'a, C> {
        self._start_date_day = Some(new_value);
        self
    }
    /// The next_page_token value returned from a previous List request, if any.
    /// This is the value of
    /// ListTrafficStatsResponse.next_page_token
    /// returned from the previous call to `ListTrafficStats` method.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DomainTrafficStatListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer TrafficStats than
    /// requested. If unspecified, server will pick an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> DomainTrafficStatListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    ///
    /// Sets the *end date.year* query property to the given value.
    pub fn end_date_year(mut self, new_value: i32) -> DomainTrafficStatListCall<'a, C> {
        self._end_date_year = Some(new_value);
        self
    }
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    ///
    /// Sets the *end date.month* query property to the given value.
    pub fn end_date_month(mut self, new_value: i32) -> DomainTrafficStatListCall<'a, C> {
        self._end_date_month = Some(new_value);
        self
    }
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    ///
    /// Sets the *end date.day* query property to the given value.
    pub fn end_date_day(mut self, new_value: i32) -> DomainTrafficStatListCall<'a, C> {
        self._end_date_day = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DomainTrafficStatListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> DomainTrafficStatListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets a specific domain registered by the client.
/// Returns NOT_FOUND if the domain does not exist.
///
/// A builder for the *get* method supported by a *domain* resource.
/// It is not used directly, but through a `DomainMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gmailpostmastertools1_beta1 as gmailpostmastertools1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2;
/// # use gmailpostmastertools1_beta1::PostmasterTools;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PostmasterTools::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.domains().get("name")
///              .doit();
/// # }
/// ```
pub struct DomainGetCall<'a, C>
    where C: 'a {

    hub: &'a PostmasterTools<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> client::CallBuilder for DomainGetCall<'a, C> {}

impl<'a, C> DomainGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Domain)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "gmailpostmastertools.domains.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+name}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The resource name of the domain. It should have the form
    /// `domains/{domain_name}`, where domain_name is the fully qualified
    /// domain name.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> DomainGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DomainGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> DomainGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists the domains that have been registered by the client. The order of
/// domains in the response is unspecified and non-deterministic. Newly created
/// domains will not necessarily be added to the end of this list.
///
/// A builder for the *list* method supported by a *domain* resource.
/// It is not used directly, but through a `DomainMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_gmailpostmastertools1_beta1 as gmailpostmastertools1_beta1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2;
/// # use gmailpostmastertools1_beta1::PostmasterTools;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PostmasterTools::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.domains().list()
///              .page_token("sed")
///              .page_size(-70)
///              .doit();
/// # }
/// ```
pub struct DomainListCall<'a, C>
    where C: 'a {

    hub: &'a PostmasterTools<C>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> client::CallBuilder for DomainListCall<'a, C> {}

impl<'a, C> DomainListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListDomainsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "gmailpostmastertools.domains.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/domains";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone());


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The next_page_token value returned from a previous List request, if any.
    /// This is the value of
    /// ListDomainsResponse.next_page_token
    /// returned from the previous call to `ListDomains` method.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DomainListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer domains than requested.
    /// If unspecified, server will pick an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> DomainListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DomainListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> DomainListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


