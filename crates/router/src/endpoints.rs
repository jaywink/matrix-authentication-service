// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};

pub use crate::traits::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "next")]
pub enum PostAuthAction {
    ContinueAuthorizationGrant {
        #[serde(deserialize_with = "serde_with::rust::display_fromstr::deserialize")]
        data: i64,
    },
    ChangePassword,
}

impl PostAuthAction {
    #[must_use]
    pub fn continue_grant(data: i64) -> Self {
        PostAuthAction::ContinueAuthorizationGrant { data }
    }

    #[must_use]
    pub fn go_next(&self) -> axum::response::Redirect {
        match self {
            Self::ContinueAuthorizationGrant { data } => ContinueAuthorizationGrant(*data).go(),
            Self::ChangePassword => AccountPassword.go(),
        }
    }
}

/// `GET /.well-known/openid-configuration`
#[derive(Debug, Clone)]
pub struct OidcConfiguration;

impl SimpleRoute for OidcConfiguration {
    const PATH: &'static str = "/.well-known/openid-configuration";
}

/// `GET /.well-known/webfinger`
#[derive(Debug, Clone)]
pub struct Webfinger;

impl SimpleRoute for Webfinger {
    const PATH: &'static str = "/.well-known/webfinger";
}

/// `GET /.well-known/change-password`
pub struct ChangePasswordDiscovery;

impl SimpleRoute for ChangePasswordDiscovery {
    const PATH: &'static str = "/.well-known/change-password";
}

/// `GET /oauth2/keys.json`
#[derive(Debug, Clone)]
pub struct OAuth2Keys;

impl SimpleRoute for OAuth2Keys {
    const PATH: &'static str = "/oauth2/keys.json";
}

/// `GET /oauth2/userinfo`
#[derive(Debug, Clone)]
pub struct OidcUserinfo;

impl SimpleRoute for OidcUserinfo {
    const PATH: &'static str = "/oauth2/userinfo";
}

/// `POST /oauth2/userinfo`
#[derive(Debug, Clone)]
pub struct OAuth2Introspection;

impl SimpleRoute for OAuth2Introspection {
    const PATH: &'static str = "/oauth2/introspect";
}

/// `POST /oauth2/token`
#[derive(Debug, Clone)]
pub struct OAuth2TokenEndpoint;

impl SimpleRoute for OAuth2TokenEndpoint {
    const PATH: &'static str = "/oauth2/token";
}

/// `POST /oauth2/registration`
#[derive(Debug, Clone)]
pub struct OAuth2RegistrationEndpoint;

impl SimpleRoute for OAuth2RegistrationEndpoint {
    const PATH: &'static str = "/oauth2/registration";
}

/// `GET /authorize`
#[derive(Debug, Clone)]
pub struct OAuth2AuthorizationEndpoint;

impl SimpleRoute for OAuth2AuthorizationEndpoint {
    const PATH: &'static str = "/authorize";
}

/// `GET /`
#[derive(Debug, Clone)]
pub struct Index;

impl SimpleRoute for Index {
    const PATH: &'static str = "/";
}

/// `GET /health`
#[derive(Debug, Clone)]
pub struct Healthcheck;

impl SimpleRoute for Healthcheck {
    const PATH: &'static str = "/health";
}

/// `GET|POST /login`
#[derive(Default, Debug, Clone)]
pub struct Login {
    post_auth_action: Option<PostAuthAction>,
}

impl Route for Login {
    type Query = PostAuthAction;

    fn route() -> &'static str {
        "/login"
    }

    fn query(&self) -> Option<&Self::Query> {
        self.post_auth_action.as_ref()
    }
}

impl Login {
    #[must_use]
    pub fn and_then(action: PostAuthAction) -> Self {
        Self {
            post_auth_action: Some(action),
        }
    }

    #[must_use]
    pub fn and_continue_grant(data: i64) -> Self {
        Self {
            post_auth_action: Some(PostAuthAction::continue_grant(data)),
        }
    }

    /// Get a reference to the login's post auth action.
    #[must_use]
    pub fn post_auth_action(&self) -> Option<&PostAuthAction> {
        self.post_auth_action.as_ref()
    }

    #[must_use]
    pub fn go_next(&self) -> axum::response::Redirect {
        match &self.post_auth_action {
            Some(action) => action.go_next(),
            None => Index.go(),
        }
    }
}

impl From<Option<PostAuthAction>> for Login {
    fn from(post_auth_action: Option<PostAuthAction>) -> Self {
        Self { post_auth_action }
    }
}

/// `POST /logout`
#[derive(Debug, Clone)]
pub struct Logout;

impl SimpleRoute for Logout {
    const PATH: &'static str = "/logout";
}

/// `GET|POST /reauth`
#[derive(Default, Debug, Clone)]
pub struct Reauth {
    post_auth_action: Option<PostAuthAction>,
}

impl Reauth {
    #[must_use]
    pub fn and_then(action: PostAuthAction) -> Self {
        Self {
            post_auth_action: Some(action),
        }
    }

    #[must_use]
    pub fn and_continue_grant(data: i64) -> Self {
        Self {
            post_auth_action: Some(PostAuthAction::continue_grant(data)),
        }
    }

    /// Get a reference to the reauth's post auth action.
    #[must_use]
    pub fn post_auth_action(&self) -> Option<&PostAuthAction> {
        self.post_auth_action.as_ref()
    }

    #[must_use]
    pub fn go_next(&self) -> axum::response::Redirect {
        match &self.post_auth_action {
            Some(action) => action.go_next(),
            None => Index.go(),
        }
    }
}

impl Route for Reauth {
    type Query = PostAuthAction;

    fn route() -> &'static str {
        "/reauth"
    }

    fn query(&self) -> Option<&Self::Query> {
        self.post_auth_action.as_ref()
    }
}

impl From<Option<PostAuthAction>> for Reauth {
    fn from(post_auth_action: Option<PostAuthAction>) -> Self {
        Self { post_auth_action }
    }
}

/// `GET|POST /register`
#[derive(Default, Debug, Clone)]
pub struct Register {
    post_auth_action: Option<PostAuthAction>,
}

impl Register {
    #[must_use]
    pub fn and_then(action: PostAuthAction) -> Self {
        Self {
            post_auth_action: Some(action),
        }
    }

    #[must_use]
    pub fn and_continue_grant(data: i64) -> Self {
        Self {
            post_auth_action: Some(PostAuthAction::continue_grant(data)),
        }
    }

    /// Get a reference to the reauth's post auth action.
    #[must_use]
    pub fn post_auth_action(&self) -> Option<&PostAuthAction> {
        self.post_auth_action.as_ref()
    }

    #[must_use]
    pub fn go_next(&self) -> axum::response::Redirect {
        match &self.post_auth_action {
            Some(action) => action.go_next(),
            None => Index.go(),
        }
    }
}

impl Route for Register {
    type Query = PostAuthAction;

    fn route() -> &'static str {
        "/register"
    }

    fn query(&self) -> Option<&Self::Query> {
        self.post_auth_action.as_ref()
    }
}

impl From<Option<PostAuthAction>> for Register {
    fn from(post_auth_action: Option<PostAuthAction>) -> Self {
        Self { post_auth_action }
    }
}

/// `GET /verify/:code`
#[derive(Debug, Clone)]
pub struct VerifyEmail(pub String);

impl Route for VerifyEmail {
    type Query = ();
    fn route() -> &'static str {
        "/verify/:code"
    }

    fn path(&self) -> std::borrow::Cow<'static, str> {
        format!("/verify/{}", self.0).into()
    }
}

/// `GET /account`
#[derive(Debug, Clone)]
pub struct Account;

impl SimpleRoute for Account {
    const PATH: &'static str = "/account";
}

/// `GET|POST /account/password`
#[derive(Debug, Clone)]
pub struct AccountPassword;

impl SimpleRoute for AccountPassword {
    const PATH: &'static str = "/account/password";
}

/// `GET|POST /account/emails`
#[derive(Debug, Clone)]
pub struct AccountEmails;

impl SimpleRoute for AccountEmails {
    const PATH: &'static str = "/account/emails";
}

/// `GET /authorize/:grant_id`
#[derive(Debug, Clone)]
pub struct ContinueAuthorizationGrant(pub i64);

impl Route for ContinueAuthorizationGrant {
    type Query = ();
    fn route() -> &'static str {
        "/authorize/:grant_id"
    }

    fn path(&self) -> std::borrow::Cow<'static, str> {
        format!("/authorize/{}", self.0).into()
    }
}

/// `GET /consent/:grant_id`
#[derive(Debug, Clone)]
pub struct Consent(pub i64);

impl Route for Consent {
    type Query = ();
    fn route() -> &'static str {
        "/consent/:grant_id"
    }

    fn path(&self) -> std::borrow::Cow<'static, str> {
        format!("/consent/{}", self.0).into()
    }
}

/// `GET|POST /_matrix/client/v3/login`
pub struct CompatLogin;

impl SimpleRoute for CompatLogin {
    const PATH: &'static str = "/_matrix/client/:version/login";
}

/// `POST /_matrix/client/v3/logout`
pub struct CompatLogout;

impl SimpleRoute for CompatLogout {
    const PATH: &'static str = "/_matrix/client/:version/logout";
}