#[macro_export]
macro_rules! v {

    () => {
        Vertex::default()
    };
    ($x:expr,,) => {
        Vertex::new($x as f32, 0.0, 0.0)
    };
    (,$y:expr,) => {
        Vertex::new(0.0, $y as f32, 0.0)
    };
    (,,$z:expr) => {
        Vertex::new(0.0, 0.0, $z as f32)
    };
    ($x:expr, $y:expr, ) => {
        Vertex::new($x as f32, $y as f32, 0.0)
    };
    ($x:expr, , $z:expr) => {
        Vertex::new($x as f32, 0.0, $z as f32)
    };
    (, $y:expr, $z:expr) => {
        Vertex::new(0.0, $y as f32, $z as f32)
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vertex::new($x as f32, $y as f32, $z as f32)
    };
}
#[macro_export]
macro_rules! edge {
    ($v1:expr, $v2:expr) => {
        Edge::new_vtx($v1, $v2)
    };
    (i$v1:expr, i$v2:expr) => {
        Edge::new_idx($v1, $v2)
    };
}