//! Bindings to the client library `libwayland-client.so`
//!
//! The generated handle is named `WAYLAND_CLIENT_HANDLE`

use {c_char, c_void, c_int, size_t, uint32_t};

use super::common::*;

pub enum wl_proxy { }
pub enum wl_display { }
pub enum wl_event_queue { }

external_library!(WaylandClient, "wayland-client",
    functions:
    // display creation and destruction
        fn wl_display_connect_to_fd(c_int) -> *mut wl_display,
        fn wl_display_connect(*const c_char) -> *mut wl_display,
        fn wl_display_disconnect(*mut wl_display) -> (),
        fn wl_display_get_fd(*mut wl_display) -> c_int,
    // display events handling
        fn wl_display_roundtrip(*mut wl_display) -> c_int,
        fn wl_display_read_events(*mut wl_display) -> c_int,
        fn wl_display_prepare_read(*mut wl_display) -> c_int,
        fn wl_display_cancel_read(*mut wl_display) -> (),
        fn wl_display_dispatch(*mut wl_display) -> c_int,
        fn wl_display_dispatch_pending(*mut wl_display) -> c_int,
    // error handling
        fn wl_display_get_error(*mut wl_display) -> c_int,
        fn wl_display_get_protocol_error(*mut wl_display, *mut *mut wl_interface, *mut uint32_t) -> uint32_t,
    // requests handling
        fn wl_display_flush(*mut wl_display) -> c_int,

    // event queues
        fn wl_event_queue_destroy(*mut wl_event_queue) -> (),
        fn wl_display_create_queue(*mut wl_display) -> *mut wl_event_queue,
        fn wl_display_roundtrip_queue(*mut wl_display, *mut wl_event_queue) -> c_int,
        fn wl_display_prepare_read_queue(*mut wl_display, *mut wl_event_queue) -> c_int,
        fn wl_display_dispatch_queue(*mut wl_display, *mut wl_event_queue) -> c_int,
        fn wl_display_dispatch_queue_pending(*mut wl_display, *mut wl_event_queue) -> c_int,

    // proxys
        fn wl_proxy_create(*mut wl_proxy, *const wl_interface) -> *mut wl_proxy,
        fn wl_proxy_destroy(*mut wl_proxy) -> (),
        fn wl_proxy_add_listener(*mut wl_proxy, *mut extern fn(), *mut c_void) -> c_int,
        fn wl_proxy_get_listener(*mut wl_proxy) -> *const c_void,
        fn wl_proxy_add_dispatcher(*mut wl_proxy, wl_dispatcher_func_t, *const c_void, *mut c_void) -> c_int,
        fn wl_proxy_marshal_array_constructor(*mut wl_proxy, uint32_t, *mut wl_argument, *const wl_interface) -> c_int,

        fn wl_proxy_marshal_array(*mut wl_proxy, uint32_t, *mut wl_argument ) -> (),
        fn wl_proxy_set_user_data(*mut wl_proxy, *mut c_void) -> (),
        fn wl_proxy_get_user_data(*mut wl_proxy) -> *mut c_void,
        fn wl_proxy_get_id(*mut wl_proxy) -> uint32_t,
        fn wl_proxy_get_class(*mut wl_proxy) -> *const c_char,
        fn wl_proxy_set_queue(*mut wl_proxy, *mut wl_event_queue) -> (),

    // log
        fn wl_log_set_handler_client(wl_log_func_t) -> (),
    // wl_log(fmt: *const c_char, ...) -> (),

    // lists
        fn wl_list_init(*mut wl_list) -> (),
        fn wl_list_insert(*mut wl_list, *mut wl_list) -> (),
        fn wl_list_remove(*mut wl_list) -> (),
        fn wl_list_length(*const wl_list) -> c_int,
        fn wl_list_empty(*const wl_list) -> c_int,
        fn wl_list_insert_list(*mut wl_list,*mut wl_list) -> (),

    // arrays
        fn wl_array_init(*mut wl_array) -> (),
        fn wl_array_release(*mut wl_array) -> (),
        fn wl_array_add(*mut wl_array,size_t) -> (),
        fn wl_array_copy(*mut wl_array, *mut wl_array) -> ()

    varargs:
        fn wl_proxy_marshal_constructor(*mut wl_proxy, uint32_t, *const wl_interface ...) -> *mut wl_proxy,
        fn wl_proxy_marshal(*mut wl_proxy, uint32_t ...) -> ()
);

#[cfg(feature = "dlopen")]
lazy_static!(
    pub static ref WAYLAND_CLIENT_OPTION: Option<WaylandClient> = { 
        WaylandClient::open("libwayland-client.so").ok()
    };
    pub static ref WAYLAND_CLIENT_HANDLE: &'static WaylandClient = {
        WAYLAND_CLIENT_OPTION.as_ref().expect("Library libwayland-client.so could not be loaded.")
    };
);

#[cfg(not(feature = "dlopen"))]
pub fn is_lib_available() -> bool { true }
#[cfg(feature = "dlopen")]
pub fn is_lib_available() -> bool { WAYLAND_CLIENT_OPTION.is_some() }