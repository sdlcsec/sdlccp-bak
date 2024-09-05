use schemars::schema_for;
use sdlc_cp_api::model::*;
use std::fs::File;
use std::io::Write;
use sdlc_cp_api::SchemaGenerator;

fn main() -> std::io::Result<()> {
    // TODO: Ideally this should be a handful of large json schemas, not one per type.
    for schema_gen in inventory::iter::<SchemaGenerator> {
        let schema = (schema_gen.generator)();
        let filename = format!("../schemas/json/{}_schema.json", schema_gen.type_name.to_lowercase());
        let mut schema_file = File::create(&filename)?;
        let schema_string = serde_json::to_string_pretty(&schema)?;
        schema_file.write_all(schema_string.as_bytes())?;
        println!("Generated schema for {} in {}", schema_gen.type_name, filename);
    }
    // FIXME: This is a workaround. I can't seem to figure out how to get the generic to work with the macro.
    generate_schema_no_macro::<SDLCRelease<phase::Development, state::Draft>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Development, state::PolicyCheckPending>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Development, state::PolicyCheckFailed>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Source, state::Draft>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Source, state::PolicyCheckPending>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Source, state::PolicyCheckFailed>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Build, state::Draft>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Build, state::PolicyCheckPending>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Build, state::PolicyCheckFailed>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Package, state::Draft>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Package, state::PolicyCheckPending>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Package, state::PolicyCheckFailed>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Deploy, state::Draft>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Deploy, state::PolicyCheckPending>>()?;
    generate_schema_no_macro::<SDLCRelease<phase::Deploy, state::PolicyCheckFailed>>()?;
    

    Ok(())
}

fn generate_schema_no_macro<T: schemars::JsonSchema>() -> std::io::Result<()> {
    let schema = schema_for!(T);
    let filename = format!("../schemas/json/{}_schema.json", std::any::type_name::<T>().to_lowercase().replace("::", "_"));
    let mut schema_file = File::create(&filename)?;
    let schema_string = serde_json::to_string_pretty(&schema)?;
    schema_file.write_all(schema_string.as_bytes())?;
    println!("Generated schema for {} in {}", std::any::type_name::<T>(), filename);
    Ok(())
}