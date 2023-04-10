mod download;
mod enums;
mod sources;

pub use download::Download;
pub use enums::Loader;
pub use sources::{curseforge::FromCurse, modrinth::FromModrinth};

use enums::{DependencyType, Sources};

type GameVersions<'v> = &'v [&'v str];
type Error = Box<dyn std::error::Error>;

/// Represents a Minecraft mod.
#[derive(Debug)]
pub struct Mod {
    /// The name of the mod
    name: String,

    /// The mod's filename
    filename: String,

    /// A URL to download the mod
    url: String,

    /// Dependencies for the mod
    dependencies: Vec<(DependencyType, String)>,

    /// Where the mod is sourced from
    source: Sources,

    /// The loader the mod is for
    loader: Loader,

    /// The version of the game the mod is for
    minecraft_version: String,
}
