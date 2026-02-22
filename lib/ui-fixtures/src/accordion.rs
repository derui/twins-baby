use leptos::prelude::*;

use ui_component::accordion::TreeAccordion;

#[component]
pub fn AccordionFixtures() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-4">
            <div data-fixture="accordion-default">
                <TreeAccordion trigger=|| view! { "Default Section" }>
                    <div>"Content inside default accordion"</div>
                </TreeAccordion>
            </div>

            <div data-fixture="accordion-initial-open">
                <TreeAccordion trigger=|| view! { "Initially Open Section" } initial_open=true>
                    <div>"Content inside initially open accordion"</div>
                </TreeAccordion>
            </div>

            <div data-fixture="accordion-nested">
                <TreeAccordion trigger=|| view! { "Parent Section" } initial_open=true>
                    <div>"Parent content"</div>
                    <TreeAccordion trigger=|| view! { "Child Section" }>
                        <div>"Nested child content"</div>
                    </TreeAccordion>
                </TreeAccordion>
            </div>
        </div>
    }
}
