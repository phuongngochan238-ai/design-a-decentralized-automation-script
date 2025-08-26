// Data model for decentralized automation script monitor

pub mod data_model {
    pub struct Script {
        id: i32,
        name: String,
        code: String,
        triggers: Vec<Trigger>,
        actions: Vec<Action>,
    }

    pub struct Trigger {
        id: i32,
        script_id: i32,
        event_type: String,
        event_data: String,
    }

    pub struct Action {
        id: i32,
        script_id: i32,
        action_type: String,
        action_data: String,
    }

    pub struct Node {
        id: i32,
        node_type: String,
        ip_address: String,
        port: i32,
        scripts: Vec<Script>,
    }

    pub struct Monitor {
        nodes: Vec<Node>,
        scripts: Vec<Script>,
    }

    impl Monitor {
        pub fn new() -> Monitor {
            Monitor {
                nodes: vec![],
                scripts: vec![],
            }
        }

        pub fn add_node(&mut self, node: Node) {
            self.nodes.push(node);
        }

        pub fn add_script(&mut self, script: Script) {
            self.scripts.push(script);
        }

        pub fn get_script(&self, id: i32) -> Option<&Script> {
            self.scripts.iter().find(|s| s.id == id)
        }

        pub fn get_node(&self, id: i32) -> Option<&Node> {
            self.nodes.iter().find(|n| n.id == id)
        }
    }
}