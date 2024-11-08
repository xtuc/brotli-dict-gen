use std::ffi::c_void;
use std::ffi::CStr;

type BoxError = Box<dyn std::error::Error>;

pub struct Dictionary(*const i8);

impl Dictionary {
    pub fn as_bytes(&self) -> &[u8] {
        let s = unsafe { CStr::from_ptr(self.0 as *mut i8) };
        s.to_bytes()
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe {
            brotli_dict_gen_sys::free_result(self.0 as *mut c_void);
        }
    }
}

pub fn generate_dict_from_files(files: Vec<Vec<u8>>) -> Result<Dictionary, BoxError> {
    let sample_sizes = files.iter().map(|file| file.len()).collect::<Vec<usize>>();
    let samples = files
        .into_iter()
        .fold(vec![], |acc, file| [acc, file].concat());

    // TODO: make sure samples is larger than blocksize

    let ret_ptr = unsafe {
        brotli_dict_gen_sys::generate(samples.as_ptr(), sample_sizes.as_ptr(), sample_sizes.len())
    };

    Ok(Dictionary(ret_ptr))
}
