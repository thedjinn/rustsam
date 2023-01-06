use std::ffi::OsStr;
use std::path::Path;

use rayon::prelude::*;
use serde::Deserialize;

use crate::Error;
use crate::serialization::deserialize_json_file;

#[derive(Deserialize)]
struct InputOutput {
    input: String,

    // Note: this field is present in the JSON but is not used by this tool.
    //output: String
}

pub fn extract_inputs<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Error> {
    let mut entries = std::fs::read_dir(path)
        .map_err(|err| Error::IOError("Could not read directory listing", Box::new(err)))?
        .map(|entry| {
            entry
                .map_err(|err| Error::IOError("Could not read next directory entry", Box::new(err)))
                .map(|entry| entry.path())
        })
        .filter(|path| match path {
            Err(_) => true,
            Ok(path) => path.extension().and_then(OsStr::to_str) == Some("json")
        })
        .collect::<Result<Vec<_>, Error>>()?
        .par_iter()
        .map(|path| {
            let entries = deserialize_json_file::<Vec<InputOutput>>(path)?;

            println!("{} entries from {}", entries.len(), path.display());

            Ok(entries)
        })
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter()
        .flatten()
        .map(|entry| entry.input)
        .collect::<Vec<_>>();

    // Sort entries by length and lexicographically for identical lengths
    entries.par_sort_unstable_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));

    Ok(entries)
}
