pub mod champion;
pub mod rune;

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Image(pub Vec<u8>);

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Assets {
    pub champion: champion::AssetMap,
    pub rune: rune::AssetMap,
}

impl Assets {
    pub fn encode(&self) -> Vec<u8> {
        let bytes = bitcode::encode(self);
        lz4_flex::compress_prepend_size(&bytes)
    }

    pub fn decode(bytes: Vec<u8>) -> Self {
        let decompressed = lz4_flex::decompress_size_prepended(&bytes).unwrap();
        bitcode::decode(&decompressed).unwrap()
    }
}
