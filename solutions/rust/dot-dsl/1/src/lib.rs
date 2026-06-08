pub mod graph {
    use std::collections::HashMap;

    use maplit::hashmap;

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: hashmap! {},
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }

            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }

            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            for node in self.nodes.iter() {
                if node.name == name {
                    return Some(node.clone());
                }
            }
            None
        }
    }

    pub mod graph_items {
        use std::collections::HashMap;

        use maplit::hashmap;

        pub mod node {

            use super::*;
            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }

                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key) {
                        Some(value) => Some(&value),
                        None => None,
                    }
                }
            }
        }

        pub mod edge {

            use super::*;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key) {
                        None => None,
                        Some(value) => Some(&value),
                    }
                }
            }
        }
    }
}