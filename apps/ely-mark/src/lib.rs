//! ElyMark: the desktop binary and the phone library share it.

mod attach;
mod autosave;
mod blocks;
mod clip;
mod compare;
mod count;
pub mod desk;
mod engine;
mod export;
mod finding;
mod folder;
mod keys;
mod look;
mod output;
mod outside;
mod paths;
mod places;
pub mod remote;
mod rename;
mod restore;
mod side;
mod text;
mod tools;
mod vcs;
mod view;

/// The activity's native thread enters here.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(activity: ely_ui::AndroidApp) {
    ely_ui::run_android(activity, ely_platform::AppId::Mark, desk::open);
}

/// The iOS host's app delegate enters here.
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn ely_ios_main() {
    ely_ui::run_ios(ely_platform::AppId::Mark, desk::open);
}
