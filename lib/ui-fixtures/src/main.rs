mod button;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::button::ButtonFixtures;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/fixtures/button") view=ButtonFixtures />
                </Routes>
            </main>
        </Router>
    }
}
