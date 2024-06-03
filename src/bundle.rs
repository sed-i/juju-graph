use serde_derive::{Deserialize, Serialize};

use crate::petgraph_wrappers::HashBackedUnGraphWithParallelEdges;

#[derive(Debug, PartialEq)]
pub struct AppRel {
    app: String,
    rel: String,
}

impl AppRel {
    pub fn from_colon_notation(colon_notation: &str) -> Self {
        // Colon-notation looks like this:
        // hydra:pg-database
        let (app, rel) = colon_notation.split_once(':').unwrap();
        Self {
            app: app.to_string(),
            rel: rel.to_string(),
        }
    }

    pub fn get_relation_label(&self, other: &Self) -> String {
        if self.rel == other.rel {
            // If both apps have the same relation name, render it only once
            self.rel.to_string()
        } else {
            // Sort lexicographically, for consistency in presentation
            let (first, second) = (
                std::cmp::min(&self.rel, &other.rel).to_string(),
                std::cmp::max(&self.rel, &other.rel).to_string(),
            );
            format!("{}:{}", first, second)
        }
    }
}

struct Relation {
    first: String,
    second: String,
    label: String,
}

impl Relation {
    pub fn from_string_pair(first: &str, second: &str) -> Self {
        let p1 = AppRel::from_colon_notation(first);
        let p2 = AppRel::from_colon_notation(second);
        let edge = p1.get_relation_label(&p2);
        Self {
            first: p1.app,
            second: p2.app,
            label: edge,
        }
    }
}

#[cfg(test)]
mod test_app_rel_pair {
    use super::*;

    #[test]
    fn test_from_colon_notation() {
        assert_eq!(
            AppRel::from_colon_notation("app:rel"),
            AppRel {
                app: "app".to_string(),
                rel: "rel".to_string()
            }
        );
        assert_eq!(
            AppRel::from_colon_notation("app_name:rel_name"),
            AppRel {
                app: "app_name".to_string(),
                rel: "rel_name".to_string()
            }
        );
        assert_eq!(
            AppRel::from_colon_notation("app-name:rel-name"),
            AppRel {
                app: "app-name".to_string(),
                rel: "rel-name".to_string()
            }
        );
    }

    #[test]
    fn test_get_relation_label() {
        let p1 = AppRel {
            app: "app-1".to_string(),
            rel: "provider".to_string(),
        };
        let p2 = AppRel {
            app: "app-2".to_string(),
            rel: "requirer".to_string(),
        };
        assert_eq!(p1.get_relation_label(&p1), "provider");
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
    pub fn to_graph(&self) -> HashBackedUnGraphWithParallelEdges<String, String> {
        let mut graph: HashBackedUnGraphWithParallelEdges<String, String> =
            HashBackedUnGraphWithParallelEdges::new();
        for [p1, p2] in &self.relations {
            let rel = Relation::from_string_pair(p1, p2);
            graph.add_edge(&rel.first, &rel.second, &rel.label);
        }

        // println!("Graph: {:?}", graph);
        graph
    }
}
