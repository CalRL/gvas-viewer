// Portions of this file are adapted from gvas2json (MIT licensed)
// Original: https://github.com/scottanderson/gvas2json

use std::env::current_dir;
use std::{fs, io};
use std::fs::{File};
use gvas::GvasFile;
use std::io::{BufReader, Cursor, Read, Write};
use serde_json::Error;
use crate::logger;

fn to_gvas(json: String) -> Result<GvasFile, Error>{
    let cursor: Cursor<String> = Cursor::new(json);
    let gvas: Result<GvasFile, Error> = from_reader(cursor);

    gvas
}

fn from_reader<R: Read>(reader: R) -> Result<GvasFile, Error> {
    let reader: BufReader<R> = BufReader::new(reader);
    Ok(serde_json::from_reader(reader)?)
}