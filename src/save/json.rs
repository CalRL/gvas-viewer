// Portions of this file are adapted from gvas2json (MIT licensed)
// Original: https://github.com/scottanderson/gvas2json

use gvas::GvasFile;

pub fn format_json(gvas: &GvasFile) -> Result<String, serde_json::Error> {
    Ok(serde_json::to_string(gvas)?)
}
