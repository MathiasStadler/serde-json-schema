use serde_json_schema::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    let schema_file = fs::read_to_string("./examples/empty_notebook.ipynb")?;
    //let address_schema = Schema::try_from(schema_file)?;
    let address_schema = Schema::try_from(schema_file)?;
    println!("{:#?}", address_schema);
    // println!("{}", serde_json::to_string_pretty(&address_schema)?);
    // println!("{:#?}", address_schema.specification().unwrap());
    // println!("{:#?}", address_schema.properties().unwrap());
    Ok(())
}
