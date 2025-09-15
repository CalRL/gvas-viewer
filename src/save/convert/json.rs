// Portions of this file are adapted from gvas2json (MIT licensed)
// Original: https://github.com/scottanderson/gvas2json

use gvas::game_version::GameVersion;
use gvas::GvasFile;
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Cursor, IsTerminal, Read, Write};
use serde::de::Error;

pub(crate) fn format_json(gvas: &GvasFile) -> Result<String, serde_json::Error> {
    Ok(serde_json::to_string(gvas)?)
}
