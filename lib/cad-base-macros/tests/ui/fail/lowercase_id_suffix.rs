use cad_base_macros::MakeId;

// Should panic: type name ends with lowercase "id" not "Id"
#[derive(MakeId)]
struct Userid(u64);

fn main() {}
