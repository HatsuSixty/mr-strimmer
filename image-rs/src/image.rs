use std::ffi::c_char;
use std::ffi::CString;

extern "C" {
    fn stbi_load(
        filename: *const c_char,
        x: *mut i32,
        y: *mut i32,
        channels_in_file: *mut i32,
        desired_channels: i32,
    ) -> *mut i8;
    fn stbi_image_free(retval_from_stbi_load: *mut i8);
}

#[allow(temporary_cstring_as_ptr)]
pub fn get_image_dimensions(image: String) -> (u32, u32) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    unsafe {
        let mut n: i32 = 0;
        let image_data: *mut c_char = stbi_load(
            CString::new(image.clone()).unwrap().as_ptr(),
            &mut width,
            &mut height,
            &mut n,
            0,
        );
        stbi_image_free(image_data);
        if image_data == 0 as *mut i8 {
            eprintln!("[get_image_dimensions] ERROR: Invalid image `{}`", image);
            return (0, 0);
        }
    }
    (width as u32, height as u32)
}
