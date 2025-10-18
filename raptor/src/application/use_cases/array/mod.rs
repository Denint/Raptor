pub mod operations {
    pub mod array_append_use_case;
    pub mod array_get_use_case;
    pub mod array_length_use_case;
    pub mod array_set_use_case;
    pub mod array_slice_use_case;
    pub mod array_update_use_case;
}

pub use operations::array_append_use_case::*;
pub use operations::array_get_use_case::*;
pub use operations::array_length_use_case::*;
pub use operations::array_set_use_case::*;
pub use operations::array_slice_use_case::*;
pub use operations::array_update_use_case::*;

pub mod builder;
pub mod controller;
