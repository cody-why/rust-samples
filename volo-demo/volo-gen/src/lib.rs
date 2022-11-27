

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod gen {
    volo::include_service!("volo_gen.rs");
}

pub use gen::volo_gen::*;
