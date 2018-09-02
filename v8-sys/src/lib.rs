#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//#[doc(inline)]
pub use self::root::v8;
pub use self::root::std as cppstd;
pub use self::root::_iobuf;
pub use self::root::__BindgenBitfieldUnit;
pub use self::root::FILE;
// #[doc(inline)]
// pub use self::root::*;