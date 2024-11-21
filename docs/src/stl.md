# Import STL files

The format of the [STL](https://en.wikipedia.org/wiki/STL_(file_format)) file. 

```rust

fn main() -> TessResult<()> {
    let building = files::stl::import_stl("examples/import_models/at_t_building.stl")?;
    Ok(())
}



```