#![allow(dead_code)]
#![allow(non_camel_case_types)]
// automatically generated by rust-bindgen
// rust-bindgen -l mpv -o src/mpv_gen.rs /usr/include/mpv/opengl_cb.h

pub type ptrdiff_t = ::std::os::raw::c_long;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __clang_max_align_nonce2: ::std::os::raw::c_double,
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type max_align_t = Struct_Unnamed1;
pub type int8_t = ::std::os::raw::c_char;
pub type int16_t = ::std::os::raw::c_short;
pub type int32_t = ::std::os::raw::c_int;
pub type int64_t = ::std::os::raw::c_long;
pub type uint8_t = ::std::os::raw::c_uchar;
pub type uint16_t = ::std::os::raw::c_ushort;
pub type uint32_t = ::std::os::raw::c_uint;
pub type uint64_t = ::std::os::raw::c_ulong;
pub type int_least8_t = ::std::os::raw::c_char;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_char;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intptr_t = ::std::os::raw::c_long;
pub type uintptr_t = ::std::os::raw::c_ulong;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub enum Struct_mpv_handle { }
pub type mpv_handle = Struct_mpv_handle;
enum_from_primitive! {
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
/// # MPV_ERROR_SUCCESS
/// No error happened (used to signal successful operation)
/// # MPV_ERROR_EVENT_QUEUE_FULL
/// The event ringbuffer is full. This means the client is choked, and can't
/// receive any events. This can happen when too many asynchronous requests
/// have been made, but not answered. Probably never happens in practice,
/// unless the mpv core is frozen for some reason, and the client keeps
/// making asynchronous requests. (Bugs in the client API implementation
/// could also trigger this, e.g. if events become "lost".)
/// # MPV_ERROR_NOMEM
/// 'Memory allocation failed' error
/// # MPV_ERROR_UNINITIALIZED
/// The mpv core wasn't configured and initialized yet. See mpv.init()
/// for additional details
/// # MPV_ERROR_INVALID_PARAMETER
/// Generic catch-all error if a parameter is set to an invalid or
/// unsupported value. This is used if there is no better error code.
///
/// Typically this is sent when you are trying to set properties or options
/// where the value is not supported, but the format itself is
///
/// # MPV_ERROR_OPTION_NOT_FOUND
/// Trying to set an option that doesn't exist.
///
/// For a full list of options, see [here](http://mpv.io/manual/master/#options)
///
/// # MPV_ERROR_OPTION_FORMAT
///
/// Trying to set an option using an unsupported format.
///
/// ## Example
///
/// Sending a 'sid' value as a float.
///
/// Note that in most of the cases, when the libmpv option/property expect an integer
/// and gets a &str, it will try to convert the said str and a integer.
///
/// # MPV_ERROR_OPTION_ERROR
/// Setting the option failed. Typically this happens if the provided option
/// value could not be parsed
/// # MPV_ERROR_PROPERTY_NOT_FOUND
/// The accessed property doesn't exist
/// # MPV_ERROR_PROPERTY_FORMAT
/// Trying to set or get a property using an unsupported MPV_FORMAT.
/// See MPV_ERROR_OPTION_FORMAT for more details.
/// # MPV_ERROR_PROPERTY_UNAVAILABLE
/// The property exists, but is not available.
///
/// ## Example
///
/// This usually happens when the
/// associated subsystem is not active, e.g. querying audio parameters while
/// audio is disabled.
/// # MPV_ERROR_PROPERTY_ERROR
/// Error setting or getting a property
/// # MPV_ERROR_COMMAND
/// General error when running a command with mpv_command and similar
/// # MPV_ERROR_LOADING_FAILED
/// Generic error on loading (used with MpvEventEndFile.error).
/// # MPV_ERROR_AO_INIT_FAILED
/// Initializing the audio output failed.
/// # MPV_ERROR_VO_INIT_FAILED
/// Initializing the video output failed.
///
/// # MPV_ERROR_NOTHING_TO_PLAY
/// There was no audio or video data to play. This also happens if the
/// file was recognized, but did not contain any audio or video streams,
/// or no streams were selected.
///
/// # MPV_ERROR_UNKNOWN_FORMAT
///
/// When trying to load the file, the file format could not be determined,
/// or the file was too broken to open it.
/// # MPV_ERROR_UNSUPPORTED
/// Generic error for signaling that certain system requirements are not
/// fulfilled.
/// # MPV_ERROR_NOT_IMPLEMENTED
/// The libmpv API function which was called is a stub only
/// Note that unimplemented mpv-rs functions will simply panic with unimplemented!()
pub enum Error {
    MPV_ERROR_SUCCESS = 0,
    MPV_ERROR_EVENT_QUEUE_FULL = -1,
    MPV_ERROR_NOMEM = -2,
    MPV_ERROR_UNINITIALIZED = -3,
    MPV_ERROR_INVALID_PARAMETER = -4,
    MPV_ERROR_OPTION_NOT_FOUND = -5,
    MPV_ERROR_OPTION_FORMAT = -6,
    MPV_ERROR_OPTION_ERROR = -7,
    MPV_ERROR_PROPERTY_NOT_FOUND = -8,
    MPV_ERROR_PROPERTY_FORMAT = -9,
    MPV_ERROR_PROPERTY_UNAVAILABLE = -10,
    MPV_ERROR_PROPERTY_ERROR = -11,
    MPV_ERROR_COMMAND = -12,
    MPV_ERROR_LOADING_FAILED = -13,
    MPV_ERROR_AO_INIT_FAILED = -14,
    MPV_ERROR_VO_INIT_FAILED = -15,
    MPV_ERROR_NOTHING_TO_PLAY = -16,
    MPV_ERROR_UNKNOWN_FORMAT = -17,
    MPV_ERROR_UNSUPPORTED = -18,
    MPV_ERROR_NOT_IMPLEMENTED = -19,
}
}
enum_from_primitive! {
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum MpvFormat {
    MPV_FORMAT_NONE = 0,
    MPV_FORMAT_STRING = 1,
    MPV_FORMAT_OSD_STRING = 2,
    MPV_FORMAT_FLAG = 3,
    MPV_FORMAT_INT64 = 4,
    MPV_FORMAT_DOUBLE = 5,
    MPV_FORMAT_NODE = 6,
    MPV_FORMAT_NODE_ARRAY = 7,
    MPV_FORMAT_NODE_MAP = 8,
    MPV_FORMAT_BYTE_ARRAY = 9,
}
}
pub type mpv_format = MpvFormat;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_node {
    pub u: Union_Unnamed2,
    pub format: mpv_format,
}
impl ::std::clone::Clone for Struct_mpv_node {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_node {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed2 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed2 {
    pub unsafe fn string(&mut self) -> *mut *mut ::std::os::raw::c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn flag(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn int64(&mut self) -> *mut int64_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn double_(&mut self) -> *mut ::std::os::raw::c_double {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn list(&mut self) -> *mut *mut Struct_mpv_node_list {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ba(&mut self) -> *mut *mut Struct_mpv_byte_array {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Union_Unnamed2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_node = Struct_mpv_node;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_node_list {
    pub num: ::std::os::raw::c_int,
    pub values: *mut mpv_node,
    pub keys: *mut *mut ::std::os::raw::c_char,
}
impl ::std::clone::Clone for Struct_mpv_node_list {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_node_list {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_node_list = Struct_mpv_node_list;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_byte_array {
    pub data: *mut ::std::os::raw::c_void,
    pub size: size_t,
}
impl ::std::clone::Clone for Struct_mpv_byte_array {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_byte_array {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_byte_array = Struct_mpv_byte_array;
enum_from_primitive! {
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum MpvEventId {
    MPV_EVENT_NONE = 0,
    MPV_EVENT_SHUTDOWN = 1,
    MPV_EVENT_LOG_MESSAGE = 2,
    MPV_EVENT_GET_PROPERTY_REPLY = 3,
    MPV_EVENT_SET_PROPERTY_REPLY = 4,
    MPV_EVENT_COMMAND_REPLY = 5,
    MPV_EVENT_START_FILE = 6,
    MPV_EVENT_END_FILE = 7,
    MPV_EVENT_FILE_LOADED = 8,
    // MPV_EVENT_TRACKS_CHANGED = 9,
    // MPV_EVENT_TRACK_SWITCHED = 10,
    MPV_EVENT_IDLE = 11,
    // MPV_EVENT_PAUSE = 12,
    // MPV_EVENT_UNPAUSE = 13,
    MPV_EVENT_TICK = 14,
    // MPV_EVENT_SCRIPT_INPUT_DISPATCH = 15,
    MPV_EVENT_CLIENT_MESSAGE = 16,
    MPV_EVENT_VIDEO_RECONFIG = 17,
    MPV_EVENT_AUDIO_RECONFIG = 18,
    // MPV_EVENT_METADATA_UPDATE = 19,
    MPV_EVENT_SEEK = 20,
    MPV_EVENT_PLAYBACK_RESTART = 21,
    MPV_EVENT_PROPERTY_CHANGE = 22,
    // MPV_EVENT_CHAPTER_CHANGE = 23,
    MPV_EVENT_QUEUE_OVERFLOW = 24,
}
}
pub type mpv_event_id = MpvEventId;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_event_property {
    pub name: *const ::std::os::raw::c_char,
    pub format: mpv_format,
    pub data: *mut ::std::os::raw::c_void,
}
impl ::std::clone::Clone for Struct_mpv_event_property {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_event_property {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_event_property = Struct_mpv_event_property;
enum_from_primitive! {
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum LogLevel {
    MPV_LOG_LEVEL_NONE = 0,
    MPV_LOG_LEVEL_FATAL = 10,
    MPV_LOG_LEVEL_ERROR = 20,
    MPV_LOG_LEVEL_WARN = 30,
    MPV_LOG_LEVEL_INFO = 40,
    MPV_LOG_LEVEL_V = 50,
    MPV_LOG_LEVEL_DEBUG = 60,
    MPV_LOG_LEVEL_TRACE = 70,
}
}
pub type mpv_log_level = LogLevel;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_event_log_message {
    pub prefix: *const ::std::os::raw::c_char,
    pub level: *const ::std::os::raw::c_char,
    pub text: *const ::std::os::raw::c_char,
    pub log_level: mpv_log_level,
}
impl ::std::clone::Clone for Struct_mpv_event_log_message {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_event_log_message {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_event_log_message = Struct_mpv_event_log_message;
enum_from_primitive! {
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum EndFileReason {
    MPV_END_FILE_REASON_EOF = 0,
    MPV_END_FILE_REASON_STOP = 2,
    MPV_END_FILE_REASON_QUIT = 3,
    MPV_END_FILE_REASON_ERROR = 4,
    MPV_END_FILE_REASON_REDIRECT = 5,
}
}
pub type mpv_end_file_reason = EndFileReason;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_event_end_file {
    pub reason: ::std::os::raw::c_int,
    pub error: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_mpv_event_end_file {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_event_end_file {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_event_end_file = Struct_mpv_event_end_file;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_event_script_input_dispatch {
    pub arg0: ::std::os::raw::c_int,
    pub _type: *const ::std::os::raw::c_char,
}
impl ::std::clone::Clone for Struct_mpv_event_script_input_dispatch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_event_script_input_dispatch {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_event_script_input_dispatch = Struct_mpv_event_script_input_dispatch;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_event_client_message {
    pub num_args: ::std::os::raw::c_int,
    pub args: *mut *const ::std::os::raw::c_char,
}
impl ::std::clone::Clone for Struct_mpv_event_client_message {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_event_client_message {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_event_client_message = Struct_mpv_event_client_message;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpv_event {
    pub event_id: mpv_event_id,
    pub error: ::std::os::raw::c_int,
    pub reply_userdata: uint64_t,
    pub data: *mut ::std::os::raw::c_void,
}
impl ::std::clone::Clone for Struct_mpv_event {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_mpv_event {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mpv_event = Struct_mpv_event;
#[derive(Clone, Copy)]
#[repr(u32)]

pub enum SubApi {
    MPV_SUB_API_OPENGL_CB = 1,
    ///
    /// Rust does not allow Enums with a single variant, hence we must add another value to the
    /// existing enum. Thus 'NOTHING' has no utility
    NOTHING,
}
pub type mpv_sub_api = SubApi;
pub enum Struct_mpv_opengl_cb_context { }
// pub type mpv_opengl_cb_context = Struct_mpv_opengl_cb_context;
pub type mpv_opengl_cb_update_fn =
    ::std::option::Option<unsafe extern "C" fn(cb_ctx: *mut ::std::os::raw::c_void)>;
pub type mpv_opengl_cb_get_proc_address_fn =
    ::std::option::Option<unsafe extern "C" fn(fn_ctx: *mut ::std::os::raw::c_void,
                                                 name: *const ::std::os::raw::c_char)
                                                 -> *mut ::std::os::raw::c_void>;
#[link(name = "mpv")]
extern "C" {
    pub fn mpv_client_api_version() -> ::std::os::raw::c_ulong;
    pub fn mpv_error_string(error: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn mpv_free(data: *mut ::std::os::raw::c_void);
    pub fn mpv_client_name(ctx: *mut mpv_handle) -> *const ::std::os::raw::c_char;
    pub fn mpv_create() -> *mut mpv_handle;
    pub fn mpv_initialize(ctx: *mut mpv_handle) -> ::std::os::raw::c_int;
    pub fn mpv_detach_destroy(ctx: *mut mpv_handle);
    pub fn mpv_terminate_destroy(ctx: *mut mpv_handle);
    pub fn mpv_create_client(ctx: *mut mpv_handle,
                             name: *const ::std::os::raw::c_char)
                             -> *mut mpv_handle;
    pub fn mpv_load_config_file(ctx: *mut mpv_handle,
                                filename: *const ::std::os::raw::c_char)
                                -> ::std::os::raw::c_int;
    pub fn mpv_suspend(ctx: *mut mpv_handle);
    pub fn mpv_resume(ctx: *mut mpv_handle);
    pub fn mpv_get_time_us(ctx: *mut mpv_handle) -> int64_t;
    pub fn mpv_free_node_contents(node: *mut mpv_node);
    pub fn mpv_set_option(ctx: *mut mpv_handle,
                          name: *const ::std::os::raw::c_char,
                          format: mpv_format,
                          data: *mut ::std::os::raw::c_void)
                          -> ::std::os::raw::c_int;
    pub fn mpv_set_option_string(ctx: *mut mpv_handle,
                                 name: *const ::std::os::raw::c_char,
                                 data: *const ::std::os::raw::c_char)
                                 -> ::std::os::raw::c_int;
    pub fn mpv_command(ctx: *mut mpv_handle,
                       args: *mut *const ::std::os::raw::c_char)
                       -> ::std::os::raw::c_int;
    pub fn mpv_command_node(ctx: *mut mpv_handle,
                            args: *mut mpv_node,
                            result: *mut mpv_node)
                            -> ::std::os::raw::c_int;
    pub fn mpv_command_string(ctx: *mut mpv_handle,
                              args: *const ::std::os::raw::c_char)
                              -> ::std::os::raw::c_int;
    pub fn mpv_command_async(ctx: *mut mpv_handle,
                             reply_userdata: uint64_t,
                             args: *mut *const ::std::os::raw::c_char)
                             -> ::std::os::raw::c_int;
    pub fn mpv_command_node_async(ctx: *mut mpv_handle,
                                  reply_userdata: uint64_t,
                                  args: *mut mpv_node)
                                  -> ::std::os::raw::c_int;
    pub fn mpv_set_property(ctx: *mut mpv_handle,
                            name: *const ::std::os::raw::c_char,
                            format: mpv_format,
                            data: *mut ::std::os::raw::c_void)
                            -> ::std::os::raw::c_int;
    pub fn mpv_set_property_string(ctx: *mut mpv_handle,
                                   name: *const ::std::os::raw::c_char,
                                   data: *const ::std::os::raw::c_char)
                                   -> ::std::os::raw::c_int;
    pub fn mpv_set_property_async(ctx: *mut mpv_handle,
                                  reply_userdata: uint64_t,
                                  name: *const ::std::os::raw::c_char,
                                  format: mpv_format,
                                  data: *mut ::std::os::raw::c_void)
                                  -> ::std::os::raw::c_int;
    pub fn mpv_get_property(ctx: *mut mpv_handle,
                            name: *const ::std::os::raw::c_char,
                            format: mpv_format,
                            data: *mut ::std::os::raw::c_void)
                            -> ::std::os::raw::c_int;
    pub fn mpv_get_property_string(ctx: *mut mpv_handle,
                                   name: *const ::std::os::raw::c_char)
                                   -> *mut ::std::os::raw::c_char;
    pub fn mpv_get_property_osd_string(ctx: *mut mpv_handle,
                                       name: *const ::std::os::raw::c_char)
                                       -> *mut ::std::os::raw::c_char;
    pub fn mpv_get_property_async(ctx: *mut mpv_handle,
                                  reply_userdata: uint64_t,
                                  name: *const ::std::os::raw::c_char,
                                  format: mpv_format)
                                  -> ::std::os::raw::c_int;
    pub fn mpv_observe_property(mpv: *mut mpv_handle,
                                reply_userdata: uint64_t,
                                name: *const ::std::os::raw::c_char,
                                format: mpv_format)
                                -> ::std::os::raw::c_int;
    pub fn mpv_unobserve_property(mpv: *mut mpv_handle,
                                  registered_reply_userdata: uint64_t)
                                  -> ::std::os::raw::c_int;
    pub fn mpv_event_name(event: mpv_event_id) -> *const ::std::os::raw::c_char;
    pub fn mpv_request_event(ctx: *mut mpv_handle,
                             event: mpv_event_id,
                             enable: ::std::os::raw::c_int)
                             -> ::std::os::raw::c_int;
    pub fn mpv_request_log_messages(ctx: *mut mpv_handle,
                                    min_level: *const ::std::os::raw::c_char)
                                    -> ::std::os::raw::c_int;
    pub fn mpv_wait_event(ctx: *mut mpv_handle,
                          timeout: ::std::os::raw::c_double)
                          -> *mut mpv_event;
    pub fn mpv_wakeup(ctx: *mut mpv_handle);
    pub fn mpv_set_wakeup_callback(ctx: *mut mpv_handle,
                                   cb:
                                       ::std::option::Option<unsafe extern "C" fn(d:
                                                                                      *mut ::std::os::raw::c_void)>,
                                   d: *mut ::std::os::raw::c_void);
    pub fn mpv_get_wakeup_pipe(ctx: *mut mpv_handle) -> ::std::os::raw::c_int;
    pub fn mpv_wait_async_requests(ctx: *mut mpv_handle);
    // pub fn mpv_get_sub_api(ctx: *mut mpv_handle,
    //                        sub_api: mpv_sub_api)
    //                        -> *mut ::std::os::raw::c_void;
    // pub fn mpv_render_context_set_update_callback(ctx: *mut mpv_opengl_cb_context,
    //                                          callback: mpv_opengl_cb_update_fn,
    //                                          callback_ctx: *mut ::std::os::raw::c_void);
    // pub fn mpv_render_context_create(ctx: *mut mpv_opengl_cb_context,
    //                              exts: *const ::std::os::raw::c_char,
    //                              get_proc_address: mpv_opengl_cb_get_proc_address_fn,
    //                              get_proc_address_ctx: *mut ::std::os::raw::c_void)
    //                              -> ::std::os::raw::c_int;
    // pub fn mpv_render_context_render(ctx: *mut mpv_opengl_cb_context,
    //                           fbo: ::std::os::raw::c_int,
    //                           w: ::std::os::raw::c_int,
    //                           h: ::std::os::raw::c_int)
    //                           -> ::std::os::raw::c_int;
    // pub fn mpv_opengl_cb_render(ctx: *mut mpv_opengl_cb_context,
    //                             fbo: ::std::os::raw::c_int,
    //                             vp: *mut ::std::os::raw::c_int)
    //                             -> ::std::os::raw::c_int;
    // pub fn mpv_render_context_report_swap(ctx: *mut mpv_opengl_cb_context,
    //                                  time: int64_t)
    //                                  -> ::std::os::raw::c_int;
    // pub fn mpv_render_context_free(ctx: *mut mpv_opengl_cb_context) -> ::std::os::raw::c_int;
}
