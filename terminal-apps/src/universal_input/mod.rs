pub mod advanced_input;
pub mod contextual_chips;
pub mod input_toolbelt;
pub mod mode_detector;
pub mod smart_features;

pub use advanced_input::{AdvancedInput, InputMode, SyntaxHighlighting};
pub use contextual_chips::{Chip, ChipType, ContextualChips};
pub use input_toolbelt::{ToolbeltItem, ToolbeltItemType, InputToolbelt};
pub use mode_detector::{ModeDetector, DetectedMode};
pub use smart_features::{SmartFeatures, Suggestion, AutoCompletion};

/// Complete Universal Input system combining all features
#[derive(Debug, Clone)]
pub struct UniversalInput {
    pub input: AdvancedInput,
    pub chips: ContextualChips,
    pub toolbelt: InputToolbelt,
    pub mode_detector: ModeDetector,
    pub smart_features: SmartFeatures,
}
