use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use deflate::write::ZlibEncoder;
use deflate::Compression;
use serde_json::json;
use std::io::Write;

pub trait MermaidRelated {
    fn to_base64(&self) -> String;
    fn to_pako(&self) -> String;
    fn to_img_url(&self) -> String;
    fn to_edit_url(&self) -> String;
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

    fn to_img_url(&self) -> String {
        format!(
            "https://mermaid.ink/img/{}",
            String::from_utf8_lossy(self).into_owned().to_base64()
        )
        // format!("https://mermaid.ink/img/pako:{}", self.to_mermaid().to_pako())
        // https://mermaid.ink/img/Z3JhcGggTFIKYW9kaC1teXNxbC1yb3V0ZXIgLS0tfGRiLXJvdXRlcnwgbXlzcWwtaW5ub2RiLWNsdXN0ZXIKYW9kaC1teXNxbC1yb3V0ZXIgLS0tfHNoYXJlZC1kYnwgYW9kaAphb2RoIC0tLXxhbXFwfCByYWJiaXRtcS1zZXJ2ZXIK
    }

    fn to_edit_url(&self) -> String {
        let spec = json!({
            "code": String::from_utf8_lossy(self),
            "mermaid": {
                "theme": "default",
            },
            // "updateEditor": false,
            // "autoSync": true,
            // "updateDiagram":false,
            // "pan":{
            //     "x":86.83623504638672,
            //     "y":83.19340515136719
            // },
            // "zoom":0.8584164770180059,
            // "editorMode":"code",
            // "panZoom":false,
        })
        .to_string();

        format!("https://mermaid.live/edit#pako:{}", spec.to_pako())
        // https://mermaid.live/edit#pako:eJx1jjsOwzAMQ69iaK4ukDN06txFjtQ6gD-xbBco4ty9cdO1I8lHghvMiQUmeCqtzlxv90iJHYZ3yR41tSpqELGz_aluzmyJMR3m7Fs53H-14kiFkW03Azixb0Ihr90oWbvUkLGIvsYMXCCIBloYpg2qkzDesTyo-Qr7_gFw70Bn
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

    fn to_img_url(&self) -> String {
        format!("https://mermaid.ink/img/{}", self.to_base64())
        // format!("https://mermaid.ink/img/pako:{}", self.to_mermaid().to_pako())
        // https://mermaid.ink/img/Z3JhcGggTFIKYW9kaC1teXNxbC1yb3V0ZXIgLS0tfGRiLXJvdXRlcnwgbXlzcWwtaW5ub2RiLWNsdXN0ZXIKYW9kaC1teXNxbC1yb3V0ZXIgLS0tfHNoYXJlZC1kYnwgYW9kaAphb2RoIC0tLXxhbXFwfCByYWJiaXRtcS1zZXJ2ZXIK
    }

    fn to_edit_url(&self) -> String {
        let spec = json!({
            "code": self,
            "mermaid": {
                "theme": "default",
            },
            // "updateEditor": false,
            // "autoSync": true,
            // "updateDiagram":false,
            // "pan":{
            //     "x":86.83623504638672,
            //     "y":83.19340515136719
            // },
            // "zoom":0.8584164770180059,
            // "editorMode":"code",
            // "panZoom":false,
        })
        .to_string();

        format!("https://mermaid.live/edit#pako:{}", spec.to_pako())
        // https://mermaid.live/edit#pako:eJx1jjsOwzAMQ69iaK4ukDN06txFjtQ6gD-xbBco4ty9cdO1I8lHghvMiQUmeCqtzlxv90iJHYZ3yR41tSpqELGz_aluzmyJMR3m7Fs53H-14kiFkW03Azixb0Ihr90oWbvUkLGIvsYMXCCIBloYpg2qkzDesTyo-Qr7_gFw70Bn
    }
}
