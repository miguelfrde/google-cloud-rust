// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate bytes;
extern crate serde;
extern crate serde_with;
extern crate std;
extern crate wkt;

/// The widget subset used by an add-on.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct AddOnWidgetSet {
    /// The list of widgets used in an add-on.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub used_widgets: std::vec::Vec<crate::model::add_on_widget_set::WidgetType>,
}

impl AddOnWidgetSet {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [used_widgets][crate::model::AddOnWidgetSet::used_widgets].
    pub fn set_used_widgets<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::add_on_widget_set::WidgetType>,
    {
        use std::iter::Iterator;
        self.used_widgets = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for AddOnWidgetSet {
    fn typename() -> &'static str {
        "type.googleapis.com/google.apps.script.type.AddOnWidgetSet"
    }
}

/// Defines additional types related to AddOnWidgetSet
pub mod add_on_widget_set {
    #[allow(unused_imports)]
    use super::*;

    /// The Widget type. DEFAULT is the basic widget set.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct WidgetType(i32);

    impl WidgetType {
        /// The default widget set.
        pub const WIDGET_TYPE_UNSPECIFIED: WidgetType = WidgetType::new(0);

        /// The date picker.
        pub const DATE_PICKER: WidgetType = WidgetType::new(1);

        /// Styled buttons include filled buttons and disabled buttons.
        pub const STYLED_BUTTONS: WidgetType = WidgetType::new(2);

        /// Persistent forms allow persisting form values during actions.
        pub const PERSISTENT_FORMS: WidgetType = WidgetType::new(3);

        /// Fixed footer in card.
        pub const FIXED_FOOTER: WidgetType = WidgetType::new(4);

        /// Update the subject and recipients of a draft.
        pub const UPDATE_SUBJECT_AND_RECIPIENTS: WidgetType = WidgetType::new(5);

        /// The grid widget.
        pub const GRID_WIDGET: WidgetType = WidgetType::new(6);

        /// A Gmail add-on action that applies to the addon compose UI.
        pub const ADDON_COMPOSE_UI_ACTION: WidgetType = WidgetType::new(7);

        /// Creates a new WidgetType instance.
        pub(crate) const fn new(value: i32) -> Self {
            Self(value)
        }

        /// Gets the enum value.
        pub fn value(&self) -> i32 {
            self.0
        }

        /// Gets the enum value as a string.
        pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
            match self.0 {
                0 => std::borrow::Cow::Borrowed("WIDGET_TYPE_UNSPECIFIED"),
                1 => std::borrow::Cow::Borrowed("DATE_PICKER"),
                2 => std::borrow::Cow::Borrowed("STYLED_BUTTONS"),
                3 => std::borrow::Cow::Borrowed("PERSISTENT_FORMS"),
                4 => std::borrow::Cow::Borrowed("FIXED_FOOTER"),
                5 => std::borrow::Cow::Borrowed("UPDATE_SUBJECT_AND_RECIPIENTS"),
                6 => std::borrow::Cow::Borrowed("GRID_WIDGET"),
                7 => std::borrow::Cow::Borrowed("ADDON_COMPOSE_UI_ACTION"),
                _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
            }
        }

        /// Creates an enum value from the value name.
        pub fn from_str_name(name: &str) -> std::option::Option<Self> {
            match name {
                "WIDGET_TYPE_UNSPECIFIED" => {
                    std::option::Option::Some(Self::WIDGET_TYPE_UNSPECIFIED)
                }
                "DATE_PICKER" => std::option::Option::Some(Self::DATE_PICKER),
                "STYLED_BUTTONS" => std::option::Option::Some(Self::STYLED_BUTTONS),
                "PERSISTENT_FORMS" => std::option::Option::Some(Self::PERSISTENT_FORMS),
                "FIXED_FOOTER" => std::option::Option::Some(Self::FIXED_FOOTER),
                "UPDATE_SUBJECT_AND_RECIPIENTS" => {
                    std::option::Option::Some(Self::UPDATE_SUBJECT_AND_RECIPIENTS)
                }
                "GRID_WIDGET" => std::option::Option::Some(Self::GRID_WIDGET),
                "ADDON_COMPOSE_UI_ACTION" => {
                    std::option::Option::Some(Self::ADDON_COMPOSE_UI_ACTION)
                }
                _ => std::option::Option::None,
            }
        }
    }

    impl std::convert::From<i32> for WidgetType {
        fn from(value: i32) -> Self {
            Self::new(value)
        }
    }

    impl std::default::Default for WidgetType {
        fn default() -> Self {
            Self::new(0)
        }
    }
}

/// Common format for declaring a  menu item, or button, that appears within a
/// host app.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct MenuItemExtensionPoint {
    /// Required. The endpoint to execute when this extension point is
    /// activated.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub run_function: std::string::String,

    /// Required. User-visible text describing the action taken by activating this
    /// extension point. For example, "Insert invoice".
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub label: std::string::String,

    /// The URL for the logo image shown in the add-on toolbar.
    ///
    /// If not set, defaults to the add-on's primary logo URL.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub logo_url: std::string::String,
}

impl MenuItemExtensionPoint {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [run_function][crate::model::MenuItemExtensionPoint::run_function].
    pub fn set_run_function<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.run_function = v.into();
        self
    }

    /// Sets the value of [label][crate::model::MenuItemExtensionPoint::label].
    pub fn set_label<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.label = v.into();
        self
    }

    /// Sets the value of [logo_url][crate::model::MenuItemExtensionPoint::logo_url].
    pub fn set_logo_url<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.logo_url = v.into();
        self
    }
}

impl wkt::message::Message for MenuItemExtensionPoint {
    fn typename() -> &'static str {
        "type.googleapis.com/google.apps.script.type.MenuItemExtensionPoint"
    }
}

/// Common format for declaring an add-on's home-page view.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct HomepageExtensionPoint {
    /// Required. The endpoint to execute when this extension point is
    /// activated.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub run_function: std::string::String,

    /// Optional. If set to `false`, disable the home-page view in this context.
    ///
    /// Defaults to `true` if unset.
    ///
    /// If an add-ons custom home-page view is disabled, an autogenerated overview
    /// card will be provided for users instead.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub enabled: std::option::Option<wkt::BoolValue>,
}

impl HomepageExtensionPoint {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [run_function][crate::model::HomepageExtensionPoint::run_function].
    pub fn set_run_function<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.run_function = v.into();
        self
    }

    /// Sets the value of [enabled][crate::model::HomepageExtensionPoint::enabled].
    pub fn set_enabled<T: std::convert::Into<std::option::Option<wkt::BoolValue>>>(
        mut self,
        v: T,
    ) -> Self {
        self.enabled = v.into();
        self
    }
}

impl wkt::message::Message for HomepageExtensionPoint {
    fn typename() -> &'static str {
        "type.googleapis.com/google.apps.script.type.HomepageExtensionPoint"
    }
}

/// Format for declaring a universal action menu item extension point.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct UniversalActionExtensionPoint {
    /// Required. User-visible text describing the action taken by activating this
    /// extension point, for example, "Add a new contact".
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub label: std::string::String,

    /// Required. The action type supported on a universal action menu item. It
    /// could be either a link to open or an endpoint to execute.
    #[serde(flatten, skip_serializing_if = "std::option::Option::is_none")]
    pub action_type:
        std::option::Option<crate::model::universal_action_extension_point::ActionType>,
}

impl UniversalActionExtensionPoint {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [label][crate::model::UniversalActionExtensionPoint::label].
    pub fn set_label<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.label = v.into();
        self
    }

    /// Sets the value of `action_type`.
    pub fn set_action_type<
        T: std::convert::Into<
                std::option::Option<crate::model::universal_action_extension_point::ActionType>,
            >,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.action_type = v.into();
        self
    }

    /// The value of [action_type][crate::model::UniversalActionExtensionPoint::action_type]
    /// if it holds a `OpenLink`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_open_link(&self) -> std::option::Option<&std::string::String> {
        #[allow(unreachable_patterns)]
        self.action_type.as_ref().and_then(|v| match v {
            crate::model::universal_action_extension_point::ActionType::OpenLink(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// The value of [action_type][crate::model::UniversalActionExtensionPoint::action_type]
    /// if it holds a `RunFunction`, `None` if the field is not set or
    /// holds a different branch.
    pub fn get_run_function(&self) -> std::option::Option<&std::string::String> {
        #[allow(unreachable_patterns)]
        self.action_type.as_ref().and_then(|v| match v {
            crate::model::universal_action_extension_point::ActionType::RunFunction(v) => {
                std::option::Option::Some(v)
            }
            _ => std::option::Option::None,
        })
    }

    /// Sets the value of [action_type][crate::model::UniversalActionExtensionPoint::action_type]
    /// to hold a `OpenLink`.
    ///
    /// Note that all the setters affecting `action_type` are
    /// mutually exclusive.
    pub fn set_open_link<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.action_type = std::option::Option::Some(
            crate::model::universal_action_extension_point::ActionType::OpenLink(v.into()),
        );
        self
    }

    /// Sets the value of [action_type][crate::model::UniversalActionExtensionPoint::action_type]
    /// to hold a `RunFunction`.
    ///
    /// Note that all the setters affecting `action_type` are
    /// mutually exclusive.
    pub fn set_run_function<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.action_type = std::option::Option::Some(
            crate::model::universal_action_extension_point::ActionType::RunFunction(v.into()),
        );
        self
    }
}

impl wkt::message::Message for UniversalActionExtensionPoint {
    fn typename() -> &'static str {
        "type.googleapis.com/google.apps.script.type.UniversalActionExtensionPoint"
    }
}

/// Defines additional types related to UniversalActionExtensionPoint
pub mod universal_action_extension_point {
    #[allow(unused_imports)]
    use super::*;

    /// Required. The action type supported on a universal action menu item. It
    /// could be either a link to open or an endpoint to execute.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub enum ActionType {
        /// URL to be opened by the UniversalAction.
        OpenLink(std::string::String),
        /// Endpoint to be run by the UniversalAction.
        RunFunction(std::string::String),
    }
}

/// Add-on configuration that is shared across all add-on host applications.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CommonAddOnManifest {
    /// Required. The display name of the add-on.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Required. The URL for the logo image shown in the add-on toolbar.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub logo_url: std::string::String,

    /// Common layout properties for the add-on cards.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub layout_properties: std::option::Option<crate::model::LayoutProperties>,

    /// The widgets used in the add-on. If this field is not specified,
    /// it indicates that default set is used.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub add_on_widget_set: std::option::Option<crate::model::AddOnWidgetSet>,

    /// Whether to pass locale information from host app.
    pub use_locale_from_app: bool,

    /// Defines an endpoint that will be executed in any context, in
    /// any host. Any cards generated by this function will always be available to
    /// the user, but may be eclipsed by contextual content when this add-on
    /// declares more targeted triggers.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub homepage_trigger: std::option::Option<crate::model::HomepageExtensionPoint>,

    /// Defines a list of extension points in the universal action menu which
    /// serves as a setting menu for the add-on. The extension point can be
    /// link URL to open or an endpoint to execute as a form
    /// submission.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub universal_actions: std::vec::Vec<crate::model::UniversalActionExtensionPoint>,

    /// An OpenLink action
    /// can only use a URL with an HTTPS, MAILTO or TEL scheme.  For HTTPS links,
    /// the URL must also
    /// [match](/gmail/add-ons/concepts/manifests#whitelisting_urls) one of the
    /// prefixes specified in this whitelist. If the prefix omits the scheme, HTTPS
    /// is assumed.  Notice that HTTP links are automatically rewritten to HTTPS
    /// links.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub open_link_url_prefixes: std::option::Option<wkt::ListValue>,
}

impl CommonAddOnManifest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::CommonAddOnManifest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [logo_url][crate::model::CommonAddOnManifest::logo_url].
    pub fn set_logo_url<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.logo_url = v.into();
        self
    }

    /// Sets the value of [layout_properties][crate::model::CommonAddOnManifest::layout_properties].
    pub fn set_layout_properties<
        T: std::convert::Into<std::option::Option<crate::model::LayoutProperties>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.layout_properties = v.into();
        self
    }

    /// Sets the value of [add_on_widget_set][crate::model::CommonAddOnManifest::add_on_widget_set].
    pub fn set_add_on_widget_set<
        T: std::convert::Into<std::option::Option<crate::model::AddOnWidgetSet>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.add_on_widget_set = v.into();
        self
    }

    /// Sets the value of [use_locale_from_app][crate::model::CommonAddOnManifest::use_locale_from_app].
    pub fn set_use_locale_from_app<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.use_locale_from_app = v.into();
        self
    }

    /// Sets the value of [homepage_trigger][crate::model::CommonAddOnManifest::homepage_trigger].
    pub fn set_homepage_trigger<
        T: std::convert::Into<std::option::Option<crate::model::HomepageExtensionPoint>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.homepage_trigger = v.into();
        self
    }

    /// Sets the value of [open_link_url_prefixes][crate::model::CommonAddOnManifest::open_link_url_prefixes].
    pub fn set_open_link_url_prefixes<
        T: std::convert::Into<std::option::Option<wkt::ListValue>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.open_link_url_prefixes = v.into();
        self
    }

    /// Sets the value of [universal_actions][crate::model::CommonAddOnManifest::universal_actions].
    pub fn set_universal_actions<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::UniversalActionExtensionPoint>,
    {
        use std::iter::Iterator;
        self.universal_actions = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for CommonAddOnManifest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.apps.script.type.CommonAddOnManifest"
    }
}

/// Card layout properties shared across all add-on host applications.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct LayoutProperties {
    /// The primary color of the add-on. It sets the color of toolbar. If no
    /// primary color is set explicitly, the default value provided by the
    /// framework is used.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub primary_color: std::string::String,

    /// The secondary color of the add-on. It sets the color of buttons.
    /// If primary color is set but no secondary color is set, the
    /// secondary color is the same as the primary color. If neither primary
    /// color nor secondary color is set, the default value provided by the
    /// framework is used.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub secondary_color: std::string::String,
}

impl LayoutProperties {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [primary_color][crate::model::LayoutProperties::primary_color].
    pub fn set_primary_color<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.primary_color = v.into();
        self
    }

    /// Sets the value of [secondary_color][crate::model::LayoutProperties::secondary_color].
    pub fn set_secondary_color<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.secondary_color = v.into();
        self
    }
}

impl wkt::message::Message for LayoutProperties {
    fn typename() -> &'static str {
        "type.googleapis.com/google.apps.script.type.LayoutProperties"
    }
}

/// Options for sending requests to add-on HTTP endpoints
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct HttpOptions {
    /// Configuration for the token sent in the HTTP Authorization header
    pub authorization_header: crate::model::HttpAuthorizationHeader,
}

impl HttpOptions {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [authorization_header][crate::model::HttpOptions::authorization_header].
    pub fn set_authorization_header<
        T: std::convert::Into<crate::model::HttpAuthorizationHeader>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.authorization_header = v.into();
        self
    }
}

impl wkt::message::Message for HttpOptions {
    fn typename() -> &'static str {
        "type.googleapis.com/google.apps.script.type.HttpOptions"
    }
}

/// Authorization header sent in add-on HTTP requests
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HttpAuthorizationHeader(i32);

impl HttpAuthorizationHeader {
    /// Default value, equivalent to `SYSTEM_ID_TOKEN`
    pub const HTTP_AUTHORIZATION_HEADER_UNSPECIFIED: HttpAuthorizationHeader =
        HttpAuthorizationHeader::new(0);

    /// Send an ID token for the project-specific Google Workspace add-ons system
    /// service account (default)
    pub const SYSTEM_ID_TOKEN: HttpAuthorizationHeader = HttpAuthorizationHeader::new(1);

    /// Send an ID token for the end user
    pub const USER_ID_TOKEN: HttpAuthorizationHeader = HttpAuthorizationHeader::new(2);

    /// Do not send an Authentication header
    pub const NONE: HttpAuthorizationHeader = HttpAuthorizationHeader::new(3);

    /// Creates a new HttpAuthorizationHeader instance.
    pub(crate) const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the enum value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Gets the enum value as a string.
    pub fn as_str_name(&self) -> std::borrow::Cow<'static, str> {
        match self.0 {
            0 => std::borrow::Cow::Borrowed("HTTP_AUTHORIZATION_HEADER_UNSPECIFIED"),
            1 => std::borrow::Cow::Borrowed("SYSTEM_ID_TOKEN"),
            2 => std::borrow::Cow::Borrowed("USER_ID_TOKEN"),
            3 => std::borrow::Cow::Borrowed("NONE"),
            _ => std::borrow::Cow::Owned(std::format!("UNKNOWN-VALUE:{}", self.0)),
        }
    }

    /// Creates an enum value from the value name.
    pub fn from_str_name(name: &str) -> std::option::Option<Self> {
        match name {
            "HTTP_AUTHORIZATION_HEADER_UNSPECIFIED" => {
                std::option::Option::Some(Self::HTTP_AUTHORIZATION_HEADER_UNSPECIFIED)
            }
            "SYSTEM_ID_TOKEN" => std::option::Option::Some(Self::SYSTEM_ID_TOKEN),
            "USER_ID_TOKEN" => std::option::Option::Some(Self::USER_ID_TOKEN),
            "NONE" => std::option::Option::Some(Self::NONE),
            _ => std::option::Option::None,
        }
    }
}

impl std::convert::From<i32> for HttpAuthorizationHeader {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl std::default::Default for HttpAuthorizationHeader {
    fn default() -> Self {
        Self::new(0)
    }
}
