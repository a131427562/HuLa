#[cfg(target_os = "ios")]
mod platform {
    unsafe extern "C" {
        fn hula_show_splashscreen();
        fn hula_hide_splashscreen();
    }

    pub fn show() {
        unsafe { hula_show_splashscreen() };
    }

    pub fn hide() {
        unsafe { hula_hide_splashscreen() };
    }
}

#[cfg(target_os = "android")]
mod platform {
    use jni::{JavaVM, objects::JObject};

    fn invoke(method: &str) -> Result<(), jni::errors::Error> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        let activity = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };

        // 直接调用Activity实例的方法，不使用静态方法
        let result = env.call_method(&activity, method, "()V", &[]);

        let _ = activity.into_raw();

        result.map(|_| ()).map_err(|err| {
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_describe();
                let _ = env.exception_clear();
            }
            err
        })
    }

    // Android 上下文在入口处尚未初始化，调用会 panic（release 为 abort）导致闪退，这里不做任何操作
    #[allow(dead_code)]
    fn invoke_unused() -> Result<(), jni::errors::Error> {
        invoke("show")
    }

    pub fn show() {}

    pub fn hide() {}
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
mod platform {
    pub fn show() {}
    pub fn hide() {}
}

pub fn show() {
    platform::show();
}

pub fn hide() {
    platform::hide();
}

/// Tauri command: 隐藏启动画面（由前端调用）
#[tauri::command]
pub fn hide_splash_screen() -> Result<(), String> {
    tracing::info!("hide_splash_screen called from frontend");
    hide();
    Ok(())
}
