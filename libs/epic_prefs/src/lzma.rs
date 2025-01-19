use std::io::Read;
use liblzma::bufread::{XzDecoder, XzEncoder};
use liblzma::stream::{LzmaOptions, Stream};

pub(crate) fn decompress_data(compressed_data: &[u8]) -> anyhow::Result<String> {
    let mut string = String::new();

    let stream = Stream::new_lzma_decoder(u64::MAX)?;
    let mut xz = XzDecoder::new_stream(compressed_data, stream);

    xz.read_to_string(&mut string)?;

    Ok(string)
}

pub(crate) fn compress_data(decompressed_data: String) -> anyhow::Result<Vec<u8>> {
    let mut output = Vec::new();

    let options = LzmaOptions::new_preset(5)?;
    let stream = Stream::new_lzma_encoder(&options)?;
    let mut xz = XzEncoder::new_stream(decompressed_data.as_bytes(), stream);

    xz.read_to_end(&mut output)?;

    output[5] = (decompressed_data.len() >> 0) as u8;
    output[6] = (decompressed_data.len() >> 8) as u8;
    output[7] = (decompressed_data.len() >> 16) as u8;
    output[8] = (decompressed_data.len() >> 24) as u8;
    output[9] = (decompressed_data.len() >> 32) as u8;
    output[10] = (decompressed_data.len() >> 40) as u8;
    output[11] = (decompressed_data.len() >> 48) as u8;
    output[12] = (decompressed_data.len() >> 56) as u8;

    Ok(output)
}