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

/// Take an HTML snapshot of a Leptos view using `to_html_branching`.
///
/// This renders the view to a static HTML string and asserts it against
/// a stored snapshot using `insta`. Set the `UPDATE_SNAPSHOT` environment
/// variable to update snapshots.
///
/// ```
/// #[tokio::test]
/// async fn test_my_component() {
///     with_leptos_owner(async {
///         use leptos::prelude::*;
///         use leptos::view;
///         let view = view! { <div>"hello"</div> };
///         assert_view_snapshot!("my_component", view);
///     }).await;
/// }
/// ```
#[macro_export]
macro_rules! assert_view_snapshot {
    ($name:expr, $view:expr) => {{
        use leptos::tachys::view::RenderHtml as _;
        let html = $view.to_html_branching();
        insta::assert_snapshot!($name, html);
    }};
}
