#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use windows::core::GUID;
#[cfg(target_os = "windows")]
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED};

#[cfg(target_os = "windows")]
#[link(name = "ole32")]
unsafe extern "system" {
    fn CoCreateInstance(
        rclsid: *const GUID,
        pUnkOuter: *mut std::ffi::c_void,
        dwClsContext: u32,
        riid: *const GUID,
        ppv: *mut *mut std::ffi::c_void,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "user32")]
unsafe extern "system" {
    fn SendMessageTimeoutW(
        hWnd: *mut std::ffi::c_void,
        Msg: u32,
        wParam: usize,
        lParam: isize,
        fuFlags: u32,
        uTimeout: u32,
        lpdwResult: *mut usize,
    ) -> isize;
}

#[cfg(target_os = "windows")]
const CLSID_POLICY_CONFIG_CLIENT: GUID = GUID::from_u128(0x870AF99C_171D_4F9E_AF0D_E63DF40C2BC9);
#[cfg(target_os = "windows")]
const IID_IPOLICY_CONFIG: GUID = GUID::from_u128(0xF8679F50_850A_41CF_9C72_430F290290C8);

#[cfg(target_os = "windows")]
const CLSID_POLICY_CONFIG_VISTA: GUID = GUID::from_u128(0x294935CE_F637_4E7C_A41B_AB255460B862);
#[cfg(target_os = "windows")]
const IID_IPOLICY_CONFIG_VISTA: GUID = GUID::from_u128(0x568B9108_44BF_40B4_9006_86AFE5B5A620);

#[cfg(target_os = "windows")]
pub fn broadcast_device_change() {
    unsafe {
        let mut result: usize = 0;
        let _ = SendMessageTimeoutW(
            0xFFFF as *mut std::ffi::c_void,
            0x0219,
            0x0007,
            0,
            0x0002,
            500,
            &mut result,
        );
        println!(" 📢 Broadcasted WM_DEVICECHANGE to force Windows apps to migrate");
    }
}

#[cfg(not(target_os = "windows"))]
pub fn broadcast_device_change() {}

#[cfg(target_os = "windows")]
pub fn set_default_audio_endpoint(device_id_str: &str) -> bool {
    if device_id_str.is_empty() {
        return false;
    }

    let wide_id: Vec<u16> = OsStr::new(device_id_str).encode_wide().chain(std::iter::once(0)).collect();

    unsafe {
        let co_init = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();

        let pairs = [
            (CLSID_POLICY_CONFIG_CLIENT, IID_IPOLICY_CONFIG),
            (CLSID_POLICY_CONFIG_VISTA, IID_IPOLICY_CONFIG_VISTA),
        ];

        let mut success = false;
        for (clsid, iid) in pairs {
            let mut ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            let hr = CoCreateInstance(&clsid, std::ptr::null_mut(), CLSCTX_INPROC_SERVER.0, &iid, &mut ptr);
            if hr == 0 && !ptr.is_null() {
                let vtable = *(ptr as *const *const *const std::ffi::c_void);
                let set_endpoint_fn_ptr = *vtable.add(13);
                let set_endpoint_fn: extern "system" fn(*mut std::ffi::c_void, *const u16, u32) -> i32 =
                    std::mem::transmute(set_endpoint_fn_ptr);

                for role in 0..3 {
                    let _ = set_endpoint_fn(ptr, wide_id.as_ptr(), role);
                }

                let release_fn_ptr = *vtable.add(2);
                let release_fn: extern "system" fn(*mut std::ffi::c_void) -> u32 =
                    std::mem::transmute(release_fn_ptr);
                let _ = release_fn(ptr);

                success = true;
                break;
            }
        }

        if co_init {
            CoUninitialize();
        }

        if success {
            println!(" ⚙️ Windows System Default Audio Endpoint switched to: {}", device_id_str);
            broadcast_device_change();
        }

        success
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_default_audio_endpoint(_device_id_str: &str) -> bool {
    false
}
