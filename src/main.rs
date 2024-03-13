use clap::Parser;
use std::fs;
use syn::{parse_file, Attribute, Item, ItemStruct};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct CliArgs {
    #[arg(short, long)]
    pub input_path: Option<String>,
}

fn check_repr(item: &Item) {
    match item {
        Item::Struct(struct_item) => {
            check_repr_attributes(&struct_item.attrs);
        }
        Item::Enum(enum_item) => {
            check_repr_attributes(&enum_item.attrs);
        }
        Item::Union(union_item) => {
            check_repr_attributes(&union_item.attrs);
        }
        item => {
            println!("Unsupported item type: {:?}", item);
        }
    }
}
fn check_repr_attributes(attrs: &[Attribute]) {
    for attr in attrs {
        dbg!(attr.parse_args_with(Attribute::parse_outer));
    }
}
pub enum RepresentationAlignments {
    C,
    Rust,
    Transparent,
    Packed,
    // U8,
    // U16,
    // U32,
    // U64,
    // I8,
    // I32,
    // I64,
}

fn process_struct(struct_item: ItemStruct) {
    check_repr(&Item::Struct(struct_item));
}

fn process_rust_script(file_path: &str) {
    // Read the contents of the file
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    // Parse the file into a syntax tree
    let syntax_tree = parse_file(&contents).expect("Failed to parse file");

    // Process items in the syntax tree
    for item in syntax_tree.items {
        match item {
            Item::Const(_) => {}
            Item::Enum(_) => {}
            Item::ExternCrate(_) => {}
            Item::Fn(_) => {}
            Item::ForeignMod(_) => {}
            Item::Impl(_) => {}
            Item::Macro(_) => {}
            Item::Mod(_) => {}
            Item::Static(_) => {}
            Item::Struct(struct_item) => {
                process_struct(struct_item)
            }
            Item::Trait(_) => {}
            Item::TraitAlias(_) => {}
            Item::Type(_) => {}
            Item::Union(_) => {}
            Item::Use(_) => {}
            Item::Verbatim(_) => {}
            _ => {}
        }
    }
}

fn main() -> Result<(), String> {
    let cli = CliArgs::parse();
    if cli.input_path.is_some() {
        process_rust_script(&cli.input_path.unwrap());
    }
    Ok(())
}
