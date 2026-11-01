//! Phones: one full-screen window; no lock, tray or updates.

use ely_platform::{AppId, startup};
use gpui::{App, Entity, Render, Window, WindowOptions};

/// Starts `app` inside the Android activity.
#[cfg(target_os = "android")]
pub fn run_android<V: Render>(
    activity: android_activity::AndroidApp,
    app: AppId,
    root: impl FnOnce(AppId, &mut Window, &mut App) -> Entity<V> + 'static,
) {
    use gpui_mobile::android::jni;
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag(app.slug()),
    );
    jni::install_panic_hook();
    jni::init_platform(&activity);
    let Some(platform) = jni::shared_platform() else {
        startup::fail(app, "the Android platform did not start");
    };
    gpui::Application::with_platform(platform.into_rc())
        .with_assets(ely_gpui_component::Assets)
        .run(move |cx| open(app, root, cx));
}

/// Starts `app` from the iOS host's app delegate.
#[cfg(target_os = "ios")]
pub fn run_ios<V: Render>(app: AppId, root: impl FnOnce(AppId, &mut Window, &mut App) -> Entity<V> + 'static) {
    gpui_mobile::ios::ffi::set_app_callback(Box::new(move |cx| open(app, root, cx)));
    gpui_mobile::ios::ffi::run_app_with_assets(ely_gpui_component::Assets);
}

fn open<V: Render>(app: AppId, root: impl FnOnce(AppId, &mut Window, &mut App) -> Entity<V> + 'static, cx: &mut App) {
    crate::opening::install_no_launch_files(cx);
    crate::run::prepare(app, cx);
    let options = WindowOptions { window_bounds: None, app_id: Some(app.slug().into()), ..Default::default() };
    let opened = cx.open_window(options, |window, cx| {
        crate::theme::follow_system(window);
        root(app, window, cx)
    });
    if let Err(error) = opened {
        startup::fail(app, &format!("the window did not open: {error:#}"));
    }
    cx.activate(true);
    log::info!("{} window open", app.name());
}
