pub mod context;
pub mod registry;

pub use crate::context::PluginContext;
pub use crate::registry::{PluginRegistry, dispatch, PluginMeta};
pub use teloxide_plugins_macros::TeloxidePlugin;

pub mod prelude {
    pub use crate::{PluginContext, dispatch, PluginMeta, TeloxidePlugin};
}