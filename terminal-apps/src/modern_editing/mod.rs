pub mod syntax_highlighter;
pub mod vim_keybindings;
pub mod autocomplete;
pub mod command_inspector;
pub mod soft_wrapping;
pub mod alias_expansion;
pub mod error_highlighting;
pub mod keyboard_bindings;
pub mod text_editor;

pub use syntax_highlighter::{SyntaxHighlighter, TokenType};
pub use vim_keybindings::{VimKeybindings, VimMode, VimMotion};
pub use autocomplete::{Autocomplete, AutocompleteType};
pub use command_inspector::{CommandInspector, InspectionResult};
pub use soft_wrapping::{SoftWrapper, WrappingInfo};
pub use alias_expansion::{AliasExpander};
pub use error_highlighting::{ErrorHighlighter, ErrorType};
pub use keyboard_bindings::{KeyBinding, KeyBindingMap};
pub use text_editor::{ModernEditor, EditorState};
