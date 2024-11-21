# Import PLY files

The [PLY](https://en.wikipedia.org/wiki/PLY_(file_format)) file format.

```rust

fn main() -> TessResult<()> {
    let building = files::ply::import_ply("examples/import_models/ply/ply_ascii.ply")?;
    Ok(())
}

```