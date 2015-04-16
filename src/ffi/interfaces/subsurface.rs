use libc::{c_void, int32_t, uint32_t};

use ffi::abi::{self, wl_proxy};

use super::surface::wl_surface;

#[repr(C)] pub struct wl_subsurface;

const WL_SUBSURFACE_DESTROY: uint32_t = 0;
const WL_SUBSURFACE_SET_POSITION: uint32_t = 1;
const WL_SUBSURFACE_PLACE_ABOVE: uint32_t = 2;
const WL_SUBSURFACE_PLACE_BELOW: uint32_t = 3;
const WL_SUBSURFACE_SET_SYNC: uint32_t = 4;
const WL_SUBSURFACE_SET_DESYNC: uint32_t = 5;

#[inline(always)]
pub unsafe fn wl_subsurface_set_user_data(subsurface: *mut wl_subsurface, data: *mut c_void) {
    abi::wl_proxy_set_user_data(subsurface as *mut wl_proxy, data)
}

#[inline(always)]
pub unsafe fn wl_subsurface_get_user_data(subsurface: *mut wl_subsurface) -> *mut c_void {
    abi::wl_proxy_get_user_data(subsurface as *mut wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_subsurface_destroy(subsurface: *mut wl_subsurface) {
    abi::wl_proxy_marshal(subsurface as *mut wl_proxy, WL_SUBSURFACE_DESTROY);
    abi::wl_proxy_destroy(subsurface as *mut wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_subsurface_set_position(subsurface: *mut wl_subsurface, x: int32_t, y: int32_t) {
    abi::wl_proxy_marshal(subsurface as *mut wl_proxy, WL_SUBSURFACE_SET_POSITION, x, y)
}

#[inline(always)]
pub unsafe fn wl_subsurface_place_above(subsurface: *mut wl_subsurface, sibling: *mut wl_surface) {
    abi::wl_proxy_marshal(subsurface as *mut wl_proxy, WL_SUBSURFACE_PLACE_ABOVE, sibling)
}

#[inline(always)]
pub unsafe fn wl_subsurface_place_below(subsurface: *mut wl_subsurface, sibling: *mut wl_surface) {
    abi::wl_proxy_marshal(subsurface as *mut wl_proxy, WL_SUBSURFACE_PLACE_BELOW, sibling)
}

#[inline(always)]
pub unsafe fn wl_subsurface_set_sync(subsurface: *mut wl_subsurface) {
    abi::wl_proxy_marshal(subsurface as *mut wl_proxy, WL_SUBSURFACE_SET_SYNC)
}

#[inline(always)]
pub unsafe fn wl_subsurface_set_desync(subsurface: *mut wl_subsurface) {
    abi::wl_proxy_marshal(subsurface as *mut wl_proxy, WL_SUBSURFACE_SET_DESYNC)
}