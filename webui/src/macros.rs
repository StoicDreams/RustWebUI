/// Macro for alerts and dialogs
#[macro_use]
pub mod dialogs;

/// Macro for classes
#[macro_use]
pub mod classes;

#[macro_use]
pub mod contexts;

#[macro_use]
pub mod helpers;

/// Macro for splitting up arguments into list items
///
/// <li>text segment</li>
#[macro_use]
pub mod list_items;

/// Macros for splitting up arguments into paragraphs
///
/// <p>text segment</p>
#[macro_use]
pub mod paragraphs;

/// Macros for syncing between threads and asynchronous processing
#[macro_use]
pub mod sync;

/// Macros for displaying section titles
#[macro_use]
pub mod titles;

pub use classes::*;
pub use contexts::*;
pub use helpers::*;
pub use list_items::*;
pub use paragraphs::*;
pub use sync::*;
pub use titles::*;
