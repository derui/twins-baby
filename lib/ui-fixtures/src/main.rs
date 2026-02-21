mod button;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::button::ButtonFixtures;

fn main() {
    mount_to_body(App);
}

#[component]
fn FixtureIndex() -> impl IntoView {
    view! {
        <nav class="p-4">
            <ul class="flex flex-col gap-2">
                <li>
                    <a
                        href="/fixtures/button"
                        class="text-blue-600 hover:text-blue-800 hover:underline font-medium"
                    >
                        "Button"
                    </a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/fixtures") view=FixtureIndex />
                    <Route path=path!("/fixtures/button") view=ButtonFixtures />
                </Routes>
            </main>
        </Router>
    }
}
