use bytes::Bytes;

pub mod champion;
pub mod emblem;
pub mod item;
pub mod rune;
pub mod spell;

#[derive(Clone, bitcode::Encode, bitcode::Decode)]
pub struct Image(Vec<u8>);

impl AsRef<[u8]> for Image {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<Vec<u8>> for Image {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<Bytes> for Image {
    fn from(bytes: Bytes) -> Self {
        Self(bytes.to_vec())
    }
}

impl From<Image> for Bytes {
    fn from(image: Image) -> Self {
        Bytes::from(image.0)
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image").finish()
    }
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Assets {
    pub champion: champion::AssetMap,
    pub rune: rune::AssetMap,
    pub spell: spell::AssetMap,
    pub item: item::AssetMap,
    pub emblem: emblem::AssetMap,
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
