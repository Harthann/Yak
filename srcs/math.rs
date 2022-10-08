use core::intrinsics;

impl f64 {

    #[rustc_allow_incoherent_impl]
    pub fn round(self) -> f64 {
        unsafe { intrinsics::roundf64(self) }
    }

    #[rustc_allow_incoherent_impl]
    pub fn trunc(self) -> f64 {
        unsafe { intrinsics::truncf64(self) }
    }

    #[rustc_allow_incoherent_impl]
    pub fn sqrtf(self) -> f64 {
        unsafe { intrinsics::sqrtf64(self) }
    }
}

impl f32 {

    #[rustc_allow_incoherent_impl]
    pub fn round(self) -> f32 {
        unsafe { intrinsics::roundf32(self) }
    }

    #[rustc_allow_incoherent_impl]
    pub fn trunc(self) -> f32 {
        unsafe { intrinsics::truncf32(self) }
    }

    #[rustc_allow_incoherent_impl]
    pub fn sqrtf(self) -> f32 {
        unsafe { intrinsics::sqrtf32(self) }
    }
}
