# Import OBJ files

The [OBJ](https://en.wikipedia.org/wiki/Wavefront_.obj_file) file format.

```rust
use tessellate::files::obj::import_objs;
fn main() -> TessResult<()> {
    let options = tobj::LoadOptions::default();
    let meshes = import_objs("path/to/your.obj", &options)?;
    Ok(())
}
```