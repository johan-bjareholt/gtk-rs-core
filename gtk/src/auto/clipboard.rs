// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SelectionData;
use crate::TextBuffer;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    pub struct Clipboard(Object<ffi::GtkClipboard>);

    match fn {
        type_ => || ffi::gtk_clipboard_get_type(),
    }
}

impl Clipboard {
    #[doc(alias = "gtk_clipboard_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::gtk_clipboard_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_clipboard_get_display")]
    pub fn display(&self) -> Option<gdk::Display> {
        unsafe { from_glib_none(ffi::gtk_clipboard_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_clipboard_get_owner")]
    pub fn owner(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_clipboard_get_owner(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gtk_clipboard_get_selection")]
    pub fn selection(&self) -> Option<gdk::Atom> {
        unsafe { from_glib_none(ffi::gtk_clipboard_get_selection(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_clipboard_request_contents")]
    pub fn request_contents<P: FnOnce(&Clipboard, &SelectionData) + 'static>(
        &self,
        target: &gdk::Atom,
        callback: P,
    ) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: FnOnce(&Clipboard, &SelectionData) + 'static>(
            clipboard: *mut ffi::GtkClipboard,
            selection_data: *mut ffi::GtkSelectionData,
            data: glib::ffi::gpointer,
        ) {
            let clipboard = from_glib_borrow(clipboard);
            let selection_data = from_glib_borrow(selection_data);
            let callback: Box_<P> = Box_::from_raw(data as *mut _);
            (*callback)(&clipboard, &selection_data);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::gtk_clipboard_request_contents(
                self.to_glib_none().0,
                target.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_clipboard_request_image")]
    pub fn request_image<P: FnOnce(&Clipboard, &gdk_pixbuf::Pixbuf) + 'static>(&self, callback: P) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: FnOnce(&Clipboard, &gdk_pixbuf::Pixbuf) + 'static>(
            clipboard: *mut ffi::GtkClipboard,
            pixbuf: *mut gdk_pixbuf::ffi::GdkPixbuf,
            data: glib::ffi::gpointer,
        ) {
            let clipboard = from_glib_borrow(clipboard);
            let pixbuf = from_glib_borrow(pixbuf);
            let callback: Box_<P> = Box_::from_raw(data as *mut _);
            (*callback)(&clipboard, &pixbuf);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::gtk_clipboard_request_image(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_clipboard_request_rich_text")]
    pub fn request_rich_text<
        P: IsA<TextBuffer>,
        Q: FnOnce(&Clipboard, &gdk::Atom, Option<&str>, usize) + 'static,
    >(
        &self,
        buffer: &P,
        callback: Q,
    ) {
        let callback_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: IsA<TextBuffer>,
            Q: FnOnce(&Clipboard, &gdk::Atom, Option<&str>, usize) + 'static,
        >(
            clipboard: *mut ffi::GtkClipboard,
            format: gdk::ffi::GdkAtom,
            text: *const libc::c_char,
            length: libc::size_t,
            data: glib::ffi::gpointer,
        ) {
            let clipboard = from_glib_borrow(clipboard);
            let format = from_glib_borrow(format);
            let text: Borrowed<Option<glib::GString>> = from_glib_borrow(text);
            let callback: Box_<Q> = Box_::from_raw(data as *mut _);
            (*callback)(&clipboard, &format, text.as_ref().as_deref(), length);
        }
        let callback = Some(callback_func::<P, Q> as _);
        let super_callback0: Box_<Q> = callback_data;
        unsafe {
            ffi::gtk_clipboard_request_rich_text(
                self.to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_clipboard_request_text")]
    pub fn request_text<P: FnOnce(&Clipboard, Option<&str>) + 'static>(&self, callback: P) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: FnOnce(&Clipboard, Option<&str>) + 'static>(
            clipboard: *mut ffi::GtkClipboard,
            text: *const libc::c_char,
            data: glib::ffi::gpointer,
        ) {
            let clipboard = from_glib_borrow(clipboard);
            let text: Borrowed<Option<glib::GString>> = from_glib_borrow(text);
            let callback: Box_<P> = Box_::from_raw(data as *mut _);
            (*callback)(&clipboard, text.as_ref().as_deref());
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::gtk_clipboard_request_text(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_clipboard_set_image")]
    pub fn set_image(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_clipboard_set_image(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_clipboard_set_text")]
    pub fn set_text(&self, text: &str) {
        let len = text.len() as i32;
        unsafe {
            ffi::gtk_clipboard_set_text(self.to_glib_none().0, text.to_glib_none().0, len);
        }
    }

    #[doc(alias = "gtk_clipboard_store")]
    pub fn store(&self) {
        unsafe {
            ffi::gtk_clipboard_store(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_clipboard_wait_for_contents")]
    pub fn wait_for_contents(&self, target: &gdk::Atom) -> Option<SelectionData> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_contents(
                self.to_glib_none().0,
                target.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_clipboard_wait_for_image")]
    pub fn wait_for_image(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe { from_glib_full(ffi::gtk_clipboard_wait_for_image(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_clipboard_wait_for_rich_text")]
    pub fn wait_for_rich_text<P: IsA<TextBuffer>>(&self, buffer: &P) -> (Vec<u8>, gdk::Atom) {
        unsafe {
            let mut format = gdk::Atom::uninitialized();
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::gtk_clipboard_wait_for_rich_text(
                    self.to_glib_none().0,
                    buffer.as_ref().to_glib_none().0,
                    format.to_glib_none_mut().0,
                    length.as_mut_ptr(),
                ),
                length.assume_init() as usize,
            );
            (ret, format)
        }
    }

    #[doc(alias = "gtk_clipboard_wait_for_targets")]
    pub fn wait_for_targets(&self) -> Option<Vec<gdk::Atom>> {
        unsafe {
            let mut targets = ptr::null_mut();
            let mut n_targets = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_clipboard_wait_for_targets(
                self.to_glib_none().0,
                &mut targets,
                n_targets.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_container_num(
                    targets,
                    n_targets.assume_init() as usize,
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_clipboard_wait_for_text")]
    pub fn wait_for_text(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gtk_clipboard_wait_for_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_clipboard_wait_for_uris")]
    pub fn wait_for_uris(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_clipboard_wait_for_uris(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_clipboard_wait_is_image_available")]
    pub fn wait_is_image_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_image_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_clipboard_wait_is_rich_text_available")]
    pub fn wait_is_rich_text_available<P: IsA<TextBuffer>>(&self, buffer: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_rich_text_available(
                self.to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_clipboard_wait_is_target_available")]
    pub fn wait_is_target_available(&self, target: &gdk::Atom) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_target_available(
                self.to_glib_none().0,
                target.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_clipboard_wait_is_text_available")]
    pub fn wait_is_text_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_text_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_clipboard_wait_is_uris_available")]
    pub fn wait_is_uris_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_uris_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_clipboard_get")]
    pub fn get(selection: &gdk::Atom) -> Clipboard {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_clipboard_get(selection.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    #[doc(alias = "gtk_clipboard_get_default")]
    pub fn default(display: &gdk::Display) -> Option<Clipboard> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_clipboard_get_default(display.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_clipboard_get_for_display")]
    pub fn for_display(display: &gdk::Display, selection: &gdk::Atom) -> Clipboard {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_for_display(
                display.to_glib_none().0,
                selection.to_glib_none().0,
            ))
        }
    }

    //pub fn connect_owner_change<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored event: Gdk.EventOwnerChange
    //}
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Clipboard")
    }
}
