pub mod commands;
#[cfg(any(test, feature = "test-support"))]
mod ending;
pub mod i18n;
#[cfg(any(test, feature = "test-support"))]
pub mod laid;
pub mod menu;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod frames;
mod opening;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod phone;
mod run;
pub mod settings;
mod shell;
pub mod tasks;
mod theme;
pub mod tray;
pub mod updates;
pub mod watching;
pub mod wheel;

#[cfg(any(test, feature = "test-support"))]
pub use ending::at_thread_end;
pub use opening::{Ask, launch_files, mail_link, on_ask, on_open};
#[cfg(any(test, feature = "test-support"))]
pub use opening::install_no_launch_files;
#[cfg(target_os = "android")]
pub use android_activity::AndroidApp;
#[cfg(target_os = "android")]
pub use phone::run_android;
#[cfg(target_os = "ios")]
pub use phone::run_ios;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use run::run;
pub use run::window_options;
pub use shell::{Root, Shell, blank, document_root, root, set_document_menus, tool_root};
pub use tasks::{LongTask, TaskState};
