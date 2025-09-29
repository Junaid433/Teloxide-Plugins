pub mod context;
pub mod registry;

pub use crate::context::PluginContext;
pub use crate::registry::{dispatch, PluginMeta, PLUGIN_REGISTRY};
pub use teloxide_plugins_macros::TeloxidePlugin;

pub mod prelude {
    pub use crate::{dispatch, PluginContext, PluginMeta, TeloxidePlugin};
}
