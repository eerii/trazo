//! Defines persistent data structures.

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::prelude::*;

/// The directory where the persistent data should be saved in.
const DATA_PATH: &str = ".data";

/// Initializes persistent data structures.
pub(super) fn plugin(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(e) = std::fs::create_dir_all(DATA_PATH) {
        warn!("Couldn't create the save directory {}: {}", DATA_PATH, e);
    };

    app.insert_resource(GameOptions::load());
}

// Resources
// ---

/// Stores options that can be configured on the menu, related to accesibility
/// and customization.
#[derive(Default, Resource, Reflect, Serialize, Deserialize, Persistent!)]
pub struct GameOptions {}

// Helpers
// ---

/// Indicates that a [Resource] can be saved and loaded from disk.
/// This is implemented automatically when using `Persistent!`.
///
/// # Examples
///
/// ```
/// # use macro_rules_attribute::derive;
/// use serde::{Deserialize, Serialize};
/// use trazo::prelude::*;
///
/// #[derive(Default, Resource, Reflect, Serialize, Deserialize, Persistent!)]
/// pub struct SomeData {
///     pub test: bool,
/// }
///
/// // The persistent data can be accessed in any system.
/// fn read(data: Res<SomeData>) {
///     info!("{:?}", data.test);
/// }
///
/// // Writing can be done in a few ways.
/// fn write(mut data: ResMut<SomeData>) {
///     // This will persist the new value.
///     data.update(|data| {
///         data.test = true;
///     });
///     // This will not until you call `persist` manually.
///     data.test = false;
///     data.persist();
/// }
/// ```
pub trait PersistentExt: Resource + Serialize + DeserializeOwned + Default + TypePath {
    /// Returns the path that this resource needs to write to.
    fn path() -> &'static str;

    /// Reads a resource from disk if it exists. If it doesn't it returns the
    /// default value.
    fn load() -> Self {
        let mut data = Self::default();
        data.reload();
        data
    }

    /// Reads the saved value of this resource and overwrites its current value.
    fn reload(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        let data = {
            let path = format!("{}/{}.toml", DATA_PATH, Self::path());
            std::fs::read_to_string(path).ok()
        };

        #[cfg(target_arch = "wasm32")]
        let data = (|| {
            let local_storage = web_sys::window()?.local_storage().ok()??;
            local_storage.get(Self::path()).ok()?
        })();

        *self = match data {
            Some(data) => toml::from_str(&data).unwrap_or_default(),
            None => Self::default(),
        };
    }

    /// Serializes the data of this resource and saves it.
    fn persist(&self) -> Result<()> {
        let name = Self::type_path();
        let data = toml::to_string(self)
            .with_context(|| format!("Failed to serialize data for {}", name))?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = format!("{}/{}.toml", DATA_PATH, Self::path());
            std::fs::write(path.clone(), data)
                .with_context(|| format!("Failed to save serialized data for {}", name))?;
        }

        #[cfg(target_arch = "wasm32")]
        {
            let local_storage = web_sys::window()
                .context("Error getting the JavaScript window")?
                .local_storage()
                .ok()
                .context("No access to localStorage")?
                .context("No access to localStorage")?;
            local_storage
                .set(Self::path(), &data)
                .ok()
                .with_context(|| format!("Failed to save serialized data for {}", name))?;
        }

        trace!("{} updated", name);
        Ok(())
    }

    /// Mutates the values of the resource using a closure and writes the result
    /// to disk after it is done.
    fn update(&mut self, f: impl Fn(&mut Self)) -> Result<()> {
        f(self);
        self.persist()
    }

    /// Returns the resource to its default value and saves it.
    fn reset(&mut self) -> Result<()> {
        *self = Self::default();
        self.persist()
    }
}

/// Declares a bevy resource that can serialize data locally and persist it
/// between game restarts.
#[macro_export]
#[doc(hidden)]
macro_rules! Persistent {
    (
        $( #[$attr:meta] )*
        $pub:vis
        struct $i:ident { $($rest:tt)* }
    ) => {
        impl PersistentExt for $i {
            #[inline]
            fn path() -> &'static str {
                stringify!($i)
            }
        }
    };
}
