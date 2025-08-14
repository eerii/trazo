//! Useful declarations grouped together for ease of use.
//! Includes modules from this crate and some redeclarations from dependencies.

pub use anyhow::Context;
pub use bevy::{
    color::palettes::css,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

#[cfg(feature = "menu")]
pub use crate::states::MenuState;
pub use crate::{
    camera::GameCamera,
    data::{GameOptions, PersistentExt},
    helpers::LaterCommandExt,
    states::{GameState, PlaySet},
    GamePlugin,
    Persistent,
};

// Shorthands for derive macros
macro_rules_attribute::derive_alias! {
    #[derive(Eq!)] = #[derive(Eq, PartialEq)];
    #[derive(Ord!)] = #[derive(Ord, PartialOrd, Eq!)];
    #[derive(Copy!)] = #[derive(Copy, Clone)];
    #[derive(Std!)] = #[derive(Debug, Copy!, Ord!, Hash)];
}
