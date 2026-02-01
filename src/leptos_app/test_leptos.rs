use any_spawner::Executor;
use leptos::prelude::Owner;
use tokio::task::LocalSet;

/// Testing helper for leptos's hook or reactives.
///
/// ```
/// #[tokio::test]
/// async fn test() {
/// with_leptos_owner(async {
///   let (now, set_now) = signal(Utc::now());
///
///   assert_eq!(now.get_untracked(), Utc::now());
///   Executor::tick().await;
///   assert_ne!(now.get_untracked(), Utc::now());
/// }).await
/// }
/// ```
pub async fn with_leptos_owner<F: Future<Output = ()>>(f: F) {
    _ = Executor::init_tokio();
    let owner = Owner::new();
    owner.set();
    LocalSet::new().run_until(f).await;
}
