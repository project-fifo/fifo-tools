/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type __int8_t = ::std::os::raw::c_char;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct __mbstate_t {
    pub _bindgen_data_: [u64; 16usize],
}
impl __mbstate_t {
    pub unsafe fn __mbstate8(&mut self)
     -> *mut [::std::os::raw::c_char; 128usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _mbstateL(&mut self) -> *mut ::std::os::raw::c_longlong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for __mbstate_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for __mbstate_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = int64_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = int64_t;
pub type user_long_t = int64_t;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = int64_t;
pub type user_off_t = int64_t;
pub type syscall_arg_t = u_int64_t;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                  *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
impl ::std::default::Default for __darwin_pthread_handler_rec {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
impl ::std::clone::Clone for _opaque_pthread_attr_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_attr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
impl ::std::clone::Clone for _opaque_pthread_cond_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_cond_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_condattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
impl ::std::clone::Clone for _opaque_pthread_mutex_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_mutex_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_mutexattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
impl ::std::default::Default for _opaque_pthread_once_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
impl ::std::clone::Clone for _opaque_pthread_rwlock_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_rwlock_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
impl ::std::default::Default for _opaque_pthread_rwlockattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
impl ::std::clone::Clone for _opaque_pthread_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _opaque_pthread_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type u_char = ::std::os::raw::c_uchar;
pub type u_short = ::std::os::raw::c_ushort;
pub type u_int = ::std::os::raw::c_uint;
pub type u_long = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint_ = ::std::os::raw::c_uint;
pub type u_quad_t = u_int64_t;
pub type quad_t = int64_t;
pub type qaddr_t = *mut quad_t;
pub type caddr_t = *mut ::std::os::raw::c_char;
pub type daddr_t = int32_t;
pub type dev_t = __darwin_dev_t;
pub type fixpt_t = u_int32_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type in_addr_t = __uint32_t;
pub type in_port_t = __uint16_t;
pub type ino_t = __darwin_ino_t;
pub type ino64_t = __darwin_ino64_t;
pub type key_t = __int32_t;
pub type mode_t = __darwin_mode_t;
pub type nlink_t = __uint16_t;
pub type id_t = __darwin_id_t;
pub type pid_t = __darwin_pid_t;
pub type off_t = __darwin_off_t;
pub type segsz_t = int32_t;
pub type swblk_t = int32_t;
pub type uid_t = __darwin_uid_t;
pub type clock_t = __darwin_clock_t;
pub type size_t = usize;
pub type ssize_t = isize;
pub type time_t = __darwin_time_t;
pub type useconds_t = __darwin_useconds_t;
pub type suseconds_t = __darwin_suseconds_t;
pub type rsize_t = __darwin_size_t;
pub type errno_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct fd_set {
    pub fds_bits: [__int32_t; 32usize],
}
impl ::std::default::Default for fd_set {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type fd_mask = __int32_t;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type pthread_cond_t = __darwin_pthread_cond_t;
pub type pthread_condattr_t = __darwin_pthread_condattr_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
pub type pthread_once_t = __darwin_pthread_once_t;
pub type pthread_rwlock_t = __darwin_pthread_rwlock_t;
pub type pthread_rwlockattr_t = __darwin_pthread_rwlockattr_t;
pub type pthread_t = __darwin_pthread_t;
pub type pthread_key_t = __darwin_pthread_key_t;
pub type fsblkcnt_t = __darwin_fsblkcnt_t;
pub type fsfilcnt_t = __darwin_fsfilcnt_t;
pub type door_ptr_t = ::std::os::raw::c_ulonglong;
pub type door_id_t = ::std::os::raw::c_ulonglong;
pub type door_attr_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct door_desc {
    pub d_attributes: door_attr_t,
    pub d_data: Union_Unnamed1,
}
impl ::std::default::Default for door_desc {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 3usize],
}
impl Union_Unnamed1 {
    pub unsafe fn d_desc(&mut self) -> *mut Struct_Unnamed2 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn d_resv(&mut self) -> *mut [::std::os::raw::c_int; 5usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed2 {
    pub d_descriptor: ::std::os::raw::c_int,
    pub d_id: door_id_t,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type door_desc_t = door_desc;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct door_info {
    pub di_target: pid_t,
    pub di_proc: door_ptr_t,
    pub di_data: door_ptr_t,
    pub di_attributes: door_attr_t,
    pub di_uniquifier: door_id_t,
    pub di_resv: [::std::os::raw::c_int; 4usize],
}
impl ::std::default::Default for door_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type door_info_t = door_info;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct door_cred {
    pub dc_euid: uid_t,
    pub dc_egid: gid_t,
    pub dc_ruid: uid_t,
    pub dc_rgid: gid_t,
    pub dc_pid: pid_t,
    pub dc_resv: [::std::os::raw::c_int; 4usize],
}
impl ::std::default::Default for door_cred {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type door_cred_t = door_cred;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct door_arg {
    pub data_ptr: *const u8,
    pub data_size: size_t,
    pub desc_ptr: *mut door_desc_t,
    pub desc_num: ::std::os::raw::c_uint,
    pub rbuf: *mut u8,
    pub rsize: size_t,
}
impl ::std::default::Default for door_arg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type door_arg_t = door_arg;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct door_results {
    pub cookie: *mut ::std::os::raw::c_void,
    pub data_ptr: *mut ::std::os::raw::c_char,
    pub data_size: size_t,
    pub desc_ptr: *mut door_desc_t,
    pub desc_num: size_t,
    pub pc: ::std::option::Option<extern "C" fn()>,
    pub nservers: ::std::os::raw::c_int,
    pub door_info: *mut door_info_t,
}
impl ::std::default::Default for door_results {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct door_return_desc {
    pub desc_ptr: *mut door_desc_t,
    pub desc_num: ::std::os::raw::c_uint,
}
impl ::std::default::Default for door_return_desc {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type door_return_desc_t = door_return_desc;
pub type door_server_procedure_t =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut ::std::os::raw::c_void,
                                               arg2:
                                                   *mut ::std::os::raw::c_char,
                                               arg3: size_t,
                                               arg4: *mut door_desc_t,
                                               arg5: ::std::os::raw::c_uint)>;
pub type door_server_func_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut door_info_t)>;
pub type door_xcreate_server_func_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut door_info_t,
                                               arg2:
                                                   ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                  *mut ::std::os::raw::c_void)
                                                                             ->
                                                                                 *mut ::std::os::raw::c_void>,
                                               arg3:
                                                   *mut ::std::os::raw::c_void,
                                               arg4:
                                                   *mut ::std::os::raw::c_void)
                              -> ::std::os::raw::c_int>;
pub type door_xcreate_thrsetup_func_t =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut ::std::os::raw::c_void)>;
#[cfg(target_os = "solaris")]
#[link(name = "door")]
extern "C" {
    pub fn door_create(arg1: door_server_procedure_t,
                       arg2: *mut ::std::os::raw::c_void,
                       arg3: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn door_revoke(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn door_info(arg1: ::std::os::raw::c_int, arg2: *mut door_info_t)
     -> ::std::os::raw::c_int;
    pub fn door_call(fd: ::std::os::raw::c_int, args: *mut door_arg_t)
     -> ::std::os::raw::c_int;
    pub fn door_return(arg1: *mut ::std::os::raw::c_char, arg2: size_t,
                       arg3: *mut door_desc_t, arg4: ::std::os::raw::c_uint)
     -> ::std::os::raw::c_int;
    pub fn door_cred(arg1: *mut door_cred_t) -> ::std::os::raw::c_int;
    pub fn door_bind(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn door_unbind() -> ::std::os::raw::c_int;
    pub fn door_getparam(arg1: ::std::os::raw::c_int,
                         arg2: ::std::os::raw::c_int, arg3: *mut size_t)
     -> ::std::os::raw::c_int;
    pub fn door_setparam(arg1: ::std::os::raw::c_int,
                         arg2: ::std::os::raw::c_int, arg3: size_t)
     -> ::std::os::raw::c_int;
    pub fn door_server_create(arg1: door_server_func_t) -> door_server_func_t;
    pub fn door_xcreate(arg1: door_server_procedure_t,
                        arg2: *mut ::std::os::raw::c_void,
                        arg3: ::std::os::raw::c_uint,
                        arg4: door_xcreate_server_func_t,
                        arg5: door_xcreate_thrsetup_func_t,
                        arg6: *mut ::std::os::raw::c_void,
                        arg7: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
