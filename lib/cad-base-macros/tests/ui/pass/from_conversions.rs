use cad_base_macros::MakeId;

#[derive(MakeId)]
struct EntityId(u64);

fn main() {
    // Test bidirectional From conversions
    let value: u64 = 42;
    let id: EntityId = value.into();
    let back: u64 = id.into();
    assert_eq!(value, back);
}
