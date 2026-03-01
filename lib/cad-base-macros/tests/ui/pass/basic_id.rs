use cad_base_macros::MakeId;

#[derive(MakeId)]
struct UserId(u64);

fn main() {
    // Test new constructor
    let id = UserId::new(42);
    assert_eq!(u64::from(id), 42);

    // Test From<u64>
    let id2: UserId = 100.into();
    assert_eq!(u64::from(id2), 100);

    // Test Display
    let id3 = UserId::new(123);
    assert_eq!(format!("{}", id3), "User123");
}
