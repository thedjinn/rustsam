use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use memmap2::Mmap;

use crate::Error;

/// Serialize the payload with bincode, compress the serialized payload with zlib, and write the
/// result to the provided writer.
pub fn serialize_to_compressed_bincode<T, W>(writer: W, payload: &T) -> Result<W, Error>
where
    T: ?Sized + serde::Serialize,
    W: Write
{
    let serialized: Vec<u8> = bincode::serialize(payload).map_err(|err|
        Error::BincodeError("Could not serialize output", err)
    )?;

    let mut encoder = ZlibEncoder::new(writer, Compression::best());

    encoder.write_all(&serialized).map_err(|err|
        Error::IOError("Could not compress data", Box::new(err))
    )?;

    encoder.finish().map_err(|err|
        Error::IOError("Could not finish compressing", Box::new(err))
    )
}

/// Read zlib-compressed bincode payload from the reader, decompress it, and then return the
/// deserialized result.
pub fn deserialize_from_compressed_bincode<T, R>(reader: R) -> Result<T, Error>
where
    T: ?Sized + serde::de::DeserializeOwned,
    R: Read
{
    bincode::deserialize_from(ZlibDecoder::new(reader)).map_err(|err|
        Error::BincodeError("Could not deserialize input", err)
    )
}

/// Read zlib-compressed bincode payload from the slice, decompress it, and then return the
/// deserialized result.
pub fn deserialize_from_compressed_bincode_slice<T>(slice: &[u8]) -> Result<T, Error>
where
    T: ?Sized + serde::de::DeserializeOwned
{
    let mut decoder = ZlibDecoder::new(slice);

    let mut uncompressed = Vec::new();
    decoder.read_to_end(&mut uncompressed).map_err(|err|
        Error::IOError("Could not decompress input", Box::new(err))
    )?;

    bincode::deserialize(&uncompressed).map_err(|err|
        Error::BincodeError("Could not deserialize input", err)
    )
}

/// Read and deserialize the JSON payload located at the provided path.
pub fn deserialize_json_file<T: for<'a> serde::Deserialize<'a>>(path: &PathBuf) -> Result<T, Error> {
    let file = File::open(path).map_err(|err| Error::IOError("Could not open reference JSON file", Box::new(err)))?;
    let mmap = unsafe { Mmap::map(&file) }.map_err(|err| Error::IOError("Could not mmap reference JSON file", Box::new(err)))?;

    serde_json::from_slice::<T>(&mmap).map_err(|err| Error::JSONError("Could not deserialize reference file", Box::new(err)))
}
