use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use deflate::write::ZlibEncoder;
use deflate::Compression;
use std::io::Write;

pub trait MermaidRelated {
    fn to_base64(&self) -> String;
    fn to_pako(&self) -> String;
}

impl MermaidRelated for &[u8] {
    fn to_base64(&self) -> String {
        URL_SAFE_NO_PAD.encode(self)
    }

    fn to_pako(&self) -> String {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(self).unwrap();
        let compressed_bytes = encoder.finish().unwrap();
        compressed_bytes.as_slice().to_base64()
    }
}

impl MermaidRelated for String {
    fn to_base64(&self) -> String {
        URL_SAFE_NO_PAD.encode(self)
    }

    fn to_pako(&self) -> String {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(self.as_bytes()).unwrap();
        let compressed_bytes = encoder.finish().unwrap();
        compressed_bytes.as_slice().to_base64()
    }
}
