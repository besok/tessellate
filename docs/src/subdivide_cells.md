#  Subdivide cells
The filter utilizes three different subdivision algorithms to subdivide a mesh’s cells:

- butterfly - Butterfly subdivision algorithm
- loop   - Loop subdivision algorithm

The running example can be found
in the <a href="https://github.com/besok/tessellate/tree/main/examples/subdivide_cells" target="_blank">repository</a>
 
```rust
fn main() -> TessResult<()> {

    let ico = mesh::shape::icosahedron::Icosahedron::create(Vertex::default(), 0.2,  Attributes::default());
    let ico_b1 = ico.subdivide_by_loop(1)?;
    let ico_b2 = ico.subdivide_by_butterfly(1)?;
    Ok()
}

```