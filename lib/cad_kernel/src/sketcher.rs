use cad_base::sketch::Sketch;

/// Sketcher derives closed surface that is basement of the kernel.
#[derive(Debug, Clone)]
pub(crate) struct Sketcher<'a> {
    sketch: &'a Sketch,
}
