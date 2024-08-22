pub struct Node {
    pub kind: NodeKind,
}

pub enum NodeKind {
    Text { text: String },
}
