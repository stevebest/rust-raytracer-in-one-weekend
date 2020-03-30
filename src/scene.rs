trait Node: std::marker::Sync {
    /// Returns a `Scene` that it belongs to.
    fn scene(&self) -> &Scene;
}

pub struct Scene {
    nodes: Vec<Box<dyn Node>>,
}
