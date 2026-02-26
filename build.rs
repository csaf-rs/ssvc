use typify::{TypeSpace, TypeSpaceSettings};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to rerun this build script if the schema file changes
    let file_path = "assets/SelectionList_2_0_0.schema.json";
    println!("cargo:rerun-if-changed={}", file_path);

    build_from_schema(file_path, "src/generated/schema")
}

fn build_from_schema(
    file_path: &str,
    target_folder: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file and deserialize the JSON schema
    let file = std::fs::File::open(file_path)?;
    let schema: schemars::schema::RootSchema = serde_json::from_reader(file)?;

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(true)
            .with_derive("PartialEq".into())
            .with_derive("Eq".into()),
    );
    type_space.add_root_schema(schema)?;

    let syn_file = syn::parse2::<syn::File>(type_space.to_stream())?;
    let content = prettyplease::unparse(&syn_file);

    let mut out_file = Path::new(&target_folder).to_path_buf();
    out_file.push("mod.rs");
    fs::create_dir_all(&target_folder)?;
    fs::write(out_file, content)?;

    Ok(())
}
