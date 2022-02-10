//! # DBus interface proxy for: `org.a11y.atspi.Collection`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Collection.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Collection")]
trait Collection {
    /// GetActiveDescendant method
    fn get_active_descendant(&self) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;

    /// GetMatches method
    fn get_matches(
        &self,
        rule: &(&[i32], i32, std::collections::HashMap<&str, &str>),
        sortby: u32,
        count: i32,
        traverse: bool,
    ) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// GetMatchesFrom method
    fn get_matches_from(
        &self,
        current_object: &zbus::zvariant::ObjectPath<'_>,
        rule: &(&[i32], i32, std::collections::HashMap<&str, &str>),
        sortby: u32,
        tree: u32,
        count: i32,
        traverse: bool,
    ) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// GetMatchesTo method
    fn get_matches_to(
        &self,
        current_object: &zbus::zvariant::ObjectPath<'_>,
        rule: &(&[i32], i32, std::collections::HashMap<&str, &str>),
        sortby: u32,
        tree: u32,
        limit_scope: bool,
        count: i32,
        traverse: bool,
    ) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;
}
