use cad_base_macro::MakeId;

// Should panic: type name doesn't end with "Id"
#[derive(MakeId)]
struct User(u64);

fn main() {}
