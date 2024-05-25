use crate::string_utils::MermaidRelated;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, PartialEq)]
pub struct AppRelPair {
    app: String,
    rel: String,
}

impl AppRelPair {
    pub fn from_colon_notation(colon_notation: &str) -> Self {
        // Colon-notation looks like this:
        // hydra:pg-database
        let (app, rel) = colon_notation.split_once(':').unwrap();
        Self { app: app.to_string(), rel: rel.to_string() }
    }

    pub fn get_relation_label(&self, other: &Self) -> String {
        if self.rel == other.rel {
            // If both apps have the same relation name, render it only once
            self.rel.to_string()
        } else {
            // Render them lexicographically
            let (first, second) = (std::cmp::min(&self.rel, &other.rel).to_string(), std::cmp::max(&self.rel, &other.rel).to_string());
            format!("{}:{}", first, second)
        }
    }
}

#[cfg(test)]
mod test_app_rel_pair {
    use super::*;

    #[test]
    fn test_from_colon_notation() {
        assert_eq!(AppRelPair::from_colon_notation("app:rel"), AppRelPair {app: "app".to_string(), rel: "rel".to_string()});
        assert_eq!(AppRelPair::from_colon_notation("app_name:rel_name"), AppRelPair {app: "app_name".to_string(), rel: "rel_name".to_string()});
        assert_eq!(AppRelPair::from_colon_notation("app-name:rel-name"), AppRelPair {app: "app-name".to_string(), rel: "rel-name".to_string()});
    }

    #[test]
    fn test_get_relation_label() {
        let p1 = AppRelPair {app: "app-1".to_string(), rel: "provider".to_string()};
        let p2 = AppRelPair {app: "app-2".to_string(), rel: "requirer".to_string()};
        assert_eq!(p1.get_relation_label(&p2), "provider:requirer");
        assert_eq!(p2.get_relation_label(&p1), "provider:requirer");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bundle {
    // A "relations" section in a bundle.yaml looks like this:
    // relations:
    //   - [hydra:pg-database, postgresql-k8s:database]
    //   - [kratos:pg-database, postgresql-k8s:database]
    relations: Vec<[String; 2]>,
}

impl Bundle {
    pub fn to_mermaid(&self) -> String {
        let mut output = String::new();
        for [rel1, rel2] in &self.relations {
            let p1 = AppRelPair::from_colon_notation(rel1);
            let p2 = AppRelPair::from_colon_notation(rel2);
            let edge = p1.get_relation_label(&p2);
            output.push_str(&format!("{} ---|{}| {}\n", p1.app, edge, p2.app));
        }

        format!("graph LR\n{}", output)
    }

    pub fn to_graphviz(&self) -> String {
        let mut output = String::new();
        for [rel1, rel2] in &self.relations {
            let p1 = AppRelPair::from_colon_notation(rel1);
            let p2 = AppRelPair::from_colon_notation(rel2);
            let edge = p1.get_relation_label(&p2);
            output.push_str(&format!("\"{}\" -- \"{}\" [label=\"{}\"]\n", p1.app, p2.app, edge));
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
