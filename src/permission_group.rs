use super::resources::{DrawableResource, Resource, StringResource, StringResourceOrString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "permission-group")]
pub struct PermissionGroup {
    /// User-readable text that describes the group. The text should be longer and more explanatory than the label.
    /// This attribute must be set as a reference to a string resource. Unlike the label attribute, it cannot be a raw string.
    #[serde(rename = "android:description")]
    pub description: Option<Resource<StringResource>>,
    /// An icon representing the permission. This attribute must be set as a reference to a drawable resource containing the image definition.
    #[serde(rename = "android:icon")]
    pub icon: Option<Resource<DrawableResource>>,
    /// A user-readable name for the group. As a convenience, the label can be directly set as a raw string while you're developing the application.
    /// However, when the application is ready to be published, it should be set as a reference to a string resource, so that it can be localized like other strings in the user interface.
    #[serde(rename = "android:label")]
    pub label: Option<StringResourceOrString>,
    /// The name of the group. This is the name that can be assigned to a `<permission>` element's `<permissionGroup>` attribute.
    #[serde(rename = "android:name")]
    pub name: Option<String>,
}