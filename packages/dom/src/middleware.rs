use floating_ui_core::{AutoPlacement as CoreAutoPlacement, Offset as CoreOffset};
use web_sys::{Element, Window};

pub use floating_ui_core::{AutoPlacementOptions, OffsetOptions, OffsetOptionsValues};

pub type AutoPlacement<'a> = CoreAutoPlacement<'a, Element, Window>;
pub type Offset = CoreOffset<Element, Window>;
