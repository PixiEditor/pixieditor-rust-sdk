#[macro_export]
macro_rules! pixieditor_extension {
    ($ext:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn load() {
            <$ext as $crate::Extension>::load();
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn initialize() {
            <$ext as $crate::Extension>::initialize();
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn user_ready() {
            <$ext as $crate::Extension>::user_ready();
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn main_window_loaded() {
            <$ext as $crate::Extension>::main_window_loaded();
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn on_user_logged_in() {
            <$ext as $crate::Extension>::user_logged_in();
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn on_user_logged_out() {
            <$ext as $crate::Extension>::user_logged_out();
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn command_invoked(name: *const c_char) {
            if name.is_null() {
                return;
            }

            let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();

            <$ext as $crate::Extension>::command_invoked(&name);
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn command_invoked_str_param(
            name_ptr: *const u8,
            name_len: i32,
            param_ptr: *const u8,
            param_len: i32,
        ) {
            unsafe {
                let name = $crate::ptr_to_str(name_ptr, name_len);
                let param = $crate::ptr_to_str(param_ptr, param_len);

                <$ext as $crate::Extension>::command_invoked_str(name, param);
            }
        }
    };
}
