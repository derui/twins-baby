use immutable::Im;
use leptos::prelude::*;


#[derive(Debug, Clone, PartialEq)]
pub struct AccordionAttrs {
    /// extracted or not
    pub extracted: Im<bool>,

    /// Role of the accordion 
    pub role: Im<&'static str>,

    _immutable: ()
}

pub struct UseAccordion {
    pub toggle: Im<Callback<()>>,
    pub open: Im<Callback<()>>,
    pub close: Im<Callback<()>>,

    /// Attributes memoized
    pub attrs: Im<Memo<AccordionAttrs>>,

    _immutable: ()
}

/// Create accordion logic
pub fn use_accordion(initial_extracted: bool) -> UseAccordion {
    let (extracted, set_extracted) = signal(initial_extracted);
    let open = Callback::new(move |_| set_extracted.set(true)).into();
    let close = Callback::new(move |_| set_extracted.set(false)).into();
    let toggle = Callback::new(move |_| set_extracted.update(|v| *v = !*v)).into();

    let attrs = Memo::new(move |_| {
        AccordionAttrs {
            extracted: extracted.get().into(),
            role: "button".into(),
            _immutable: ()
        }
    }).into();

    UseAccordion { toggle, open, close, attrs, _immutable: () }
}

