use core::ffi::c_void;
use ndr_proxy::Guid;
#[cfg(has_proxy_blob)]
use ndr_proxy::Tables;

#[allow(dead_code)]
mod table;

#[cfg(has_proxy_blob)]
pub static TABLES: Tables = Tables {
    blob: include_bytes!("../data/proxy_19041.bin"),
    proxy_file_list: table::PROXY_FILE_LIST,
    clsid: table::CLSID,
    factory: table::FACTORY,
    files: table::FILES,
    dlls: table::DLLS,
    fixups: table::FIXUPS,
    rdata_len: table::RDATA_LEN,
};

pub const SOURCE_RVA: u32 = table::SOURCE_RVA;
pub const FILE_COUNT: usize = table::FILE_COUNT;

#[no_mangle]
pub unsafe extern "system" fn GetProxyDllInfo(info: *mut *const c_void, id: *mut *const Guid) {
    #[cfg(has_proxy_blob)]
    ndr_proxy::get_proxy_dll_info(&TABLES, info, id);
    #[cfg(not(has_proxy_blob))]
    {
        if !info.is_null() {
            *info = core::ptr::null();
        }
        if !id.is_null() {
            *id = core::ptr::null();
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn DllGetClassObject(clsid: *const Guid, iid: *const Guid, ppv: *mut *mut c_void) -> i32 {
    #[cfg(has_proxy_blob)]
    return ndr_proxy::dll_get_class_object(&TABLES, clsid, iid, ppv);
    #[cfg(not(has_proxy_blob))]
    {
        let _ = (clsid, iid, ppv);
        ndr_proxy::CLASS_E_CLASSNOTAVAILABLE
    }
}

#[no_mangle]
pub unsafe extern "system" fn DllCanUnloadNow() -> i32 {
    #[cfg(has_proxy_blob)]
    return ndr_proxy::dll_can_unload_now(&TABLES);
    #[cfg(not(has_proxy_blob))]
    0
}

#[no_mangle]
pub unsafe extern "system" fn DllRegisterServer() -> i32 {
    #[cfg(has_proxy_blob)]
    return ndr_proxy::dll_register_server(&TABLES);
    #[cfg(not(has_proxy_blob))]
    0
}

#[no_mangle]
pub unsafe extern "system" fn DllUnregisterServer() -> i32 {
    #[cfg(has_proxy_blob)]
    return ndr_proxy::dll_unregister_server(&TABLES);
    #[cfg(not(has_proxy_blob))]
    0
}
