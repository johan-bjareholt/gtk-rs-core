// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ObjectFactory;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Registry(Object<ffi::AtkRegistry, ffi::AtkRegistryClass>);

    match fn {
        type_ => || ffi::atk_registry_get_type(),
    }
}

pub const NONE_REGISTRY: Option<&Registry> = None;

pub trait RegistryExt: 'static {
    #[doc(alias = "atk_registry_get_factory")]
    fn factory(&self, type_: glib::types::Type) -> Option<ObjectFactory>;

    #[doc(alias = "atk_registry_get_factory_type")]
    fn factory_type(&self, type_: glib::types::Type) -> glib::types::Type;

    #[doc(alias = "atk_registry_set_factory_type")]
    fn set_factory_type(&self, type_: glib::types::Type, factory_type: glib::types::Type);
}

impl<O: IsA<Registry>> RegistryExt for O {
    fn factory(&self, type_: glib::types::Type) -> Option<ObjectFactory> {
        unsafe {
            from_glib_none(ffi::atk_registry_get_factory(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    fn factory_type(&self, type_: glib::types::Type) -> glib::types::Type {
        unsafe {
            from_glib(ffi::atk_registry_get_factory_type(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    fn set_factory_type(&self, type_: glib::types::Type, factory_type: glib::types::Type) {
        unsafe {
            ffi::atk_registry_set_factory_type(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
                factory_type.to_glib(),
            );
        }
    }
}

impl fmt::Display for Registry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Registry")
    }
}
