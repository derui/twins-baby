use cad_base_macros::MakeId;

#[derive(MakeId)]
struct UserAccountId(u64);

fn main() {
    // Test new constructor
    let id = UserAccountId::new(999);
    assert_eq!(u64::from(id), 999);

    // Test Display with multi-word type
    let id2 = UserAccountId::new(999);
    assert_eq!(format!("{}", id2), "UserAccount999");
}
