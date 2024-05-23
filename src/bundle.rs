use crate::string_utils::MermaidRelated;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bundle {
    relations: Vec<[String; 2]>,
}

impl Bundle {
    pub fn to_mermaid(&self) -> String {
        let mut mermaid = String::new();
        for [rel1, rel2] in &self.relations {
            let mut s1 = rel1.split(':');
            let app1 = s1.next().unwrap();
            let relname1 = s1.next().unwrap();

            let mut s2 = rel2.split(':');
            let app2 = s2.next().unwrap();
            let relname2 = s2.next().unwrap();

            // If both charms have the same relation name, render it only once
            let edge = if relname1 == relname2 {
                relname1.to_string()
            } else {
                format!("{}:{}", relname1, relname2)
            };

            mermaid.push_str(&format!("{} ---|{}| {}\n", app1, edge, app2));
        }

        format!("graph LR\n{}", mermaid)
    }

    pub fn to_graphviz(&self) -> String {
        let mut output = String::new();
        for [rel1, rel2] in &self.relations {
            let mut s1 = rel1.split(':');
            let app1 = s1.next().unwrap();
            let relname1 = s1.next().unwrap();

            let mut s2 = rel2.split(':');
            let app2 = s2.next().unwrap();
            let relname2 = s2.next().unwrap();

            // If both charms have the same relation name, render it only once
            let edge = if relname1 == relname2 {
                relname1.to_string()
            } else {
                format!("{}:{}", relname1, relname2)
            };

            output.push_str(&format!("\"{}\" -- \"{}\" [label=\"{}\"]\n", app1, app2, edge));
        }

        // Could add rankdir=LR at the top, but diagram looks better without it.
        format!("graph {{\n{}}}", output)
    }

    pub fn to_img_url(&self) -> String {
        format!("https://mermaid.ink/img/{}", self.to_mermaid().to_base64())
        // format!("https://mermaid.ink/img/pako:{}", self.to_mermaid().to_pako())
        // https://mermaid.ink/img/Z3JhcGggTFIKYW9kaC1teXNxbC1yb3V0ZXIgLS0tfGRiLXJvdXRlcnwgbXlzcWwtaW5ub2RiLWNsdXN0ZXIKYW9kaC1teXNxbC1yb3V0ZXIgLS0tfHNoYXJlZC1kYnwgYW9kaAphb2RoIC0tLXxhbXFwfCByYWJiaXRtcS1zZXJ2ZXIK
    }

    pub fn to_edit_url(&self) -> String {
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
