use std::mem;

use windows::{
    core::{ComInterface, Interface, GUID, PCWSTR},
    Win32::{
        Media::{
            DirectShow::{IAMCameraControl, IBaseFilter, ICreateDevEnum},
            MediaFoundation::{CLSID_SystemDeviceEnum, CLSID_VideoInputDeviceCategory},
        },
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx, CreateBindCtx, IEnumMoniker, IMoniker,
                StructuredStorage::IPropertyBag, CLSCTX_ALL, COINIT_MULTITHREADED,
            },
            Variant::VARIANT,
        },
    },
};

use crate::consts::camctl::*;
use crate::consts::guids::*;


fn main() {
    windows_1();
}

fn windows_1() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).expect("initialization");

        let enum_dev: ICreateDevEnum =
            CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_ALL).expect("devEnum"); // works
        println!("enum_dev: {:?}", enum_dev);

        let mut class_enumerator: Option<IEnumMoniker> = None;
        enum_dev
            .CreateClassEnumerator(&CLSID_VideoInputDeviceCategory, &mut class_enumerator, 0)
            .expect("CreateClassEnumerator");
        println!("class_enumerator: {:?}", class_enumerator);

        if class_enumerator.is_none() {
            println!("no devices in category");
            return;
        }
        let enum_moniker = class_enumerator.unwrap();

        let mut monikers: Vec<Option<IMoniker>> = Vec::new();
        monikers.push(None);

        let celt: Option<*mut u32> = None;
        for n in 0..10 {
            let hr = enum_moniker.Next(&mut monikers, celt);

            match hr.0 {
                0 => {
                    // ok
                    let current = monikers.get(0).expect("current");
                    println!("[{n}] found: {:?}", current);

                    let _bind_ctx = CreateBindCtx(0).expect("bindctx");
                    
                    let m = current.as_ref().unwrap();
                    println!(" --> m: {:?}", m);
                    
                    // ### PropertyBag ###########################################################################################################################################
                    let mut result__ = ::std::ptr::null_mut();
                    let mut result__2 = ::std::ptr::null_mut();
                    let hr = m.BindToStorage(None, None, &P_BAG, &mut result__);
                    println!(" --> IPropertyBag as Storage: {:?}", hr);
                    let bag: IPropertyBag = IPropertyBag::from_raw(result__.as_mut().unwrap());
                    println!("   - bag: {:?}", bag);

                    // ### PropertyBag->VARIANT ##################################################################################################################################
                    let mut filter: Vec<u16> = "FriendlyName".encode_utf16().collect();
                    filter.push(0); // EOF
                    let pfilter: *const u16 = filter.as_ptr();
                    let property_name: PCWSTR = PCWSTR::from_raw(pfilter);

                    let prop_val: *mut VARIANT = &mut Default::default(); // The system cannot find the file specified.

                    let read = bag.Read(property_name, prop_val, None);
                    println!("   - read: {:?}", read);
                    println!("   - pvar: {:?}", prop_val);
                    let val = &(*prop_val).Anonymous.Anonymous;
                    println!("   - bstrVal: {:?}", val.Anonymous.bstrVal);
                    // no idea if required...
                    mem::forget(val);
                    mem::forget(prop_val);
                    mem::forget(property_name);
                    mem::forget(pfilter);

                    // ###################################################################################################################################################
                    let hr = m.BindToObject(None, None, &BF, &mut result__);
                    println!(" --> IBaseFilter: {:?}  --  result__: {:?}", hr, result__);
                    
                    // ###################################################################################################################################################
                    let hr = m.BindToObject(None, None, &IAMCC, &mut result__2);
                    println!(" --> IAMCameraControl: {:?}  --  result__: {:?}", hr, result__2);
                    if hr.is_ok() {
                        let camctl: IAMCameraControl = IAMCameraControl::from_raw(result__2.as_mut().unwrap());
                        println!("   - camctl: {:?}", camctl);
                        
                        let val: *mut i32 = mem::zeroed();
                        let flags: *mut i32 = mem::zeroed();
                        let mut the_prop = IAMCameraControl_PROP_ROLL;
                        for m in 0..10 {
                            the_prop = m;
                            let res = camctl.Get(the_prop, val, flags);
                            println!("   - camprop({}): val={:?}, flags={:?} ({:?})", the_prop, val, flags, res);
                        }

                        // let res = camctl.Set(IAMCameraControl_FOCUS, IAMCameraControl_FOCUS_OFF, 0);
                        // println!("   - camctl.set(0): {:?}", res);
                    }
                }
                1 => {
                    println!("end.");
                    return;
                }
                _ => {
                    println!("error: {hr}");
                    return;
                }
            }
        }
    }
}

#[allow(dead_code, non_upper_case_globals)]
mod consts {
    pub mod camctl {
        pub const IAMCameraControl_PROP_PAN: i32 = 0;
        pub const IAMCameraControl_PROP_TILT: i32 = 1;
        pub const IAMCameraControl_PROP_ROLL: i32 = 2;
        pub const IAMCameraControl_PROP_ZOOM: i32 = 3;
        pub const IAMCameraControl_PROP_EXPOSURE: i32 = 4;
        pub const IAMCameraControl_PROP_FOCUS: i32 = 6;
        pub const IAMCameraControl_FLAG_FOCUS_NONE: i32 = 0x0000;
        pub const IAMCameraControl_FLAG_FOCUS_AUTO: i32 = 0x0001;
        pub const IAMCameraControl_FLAG_FOCUS_MANUAL: i32 = 0x0002;
    }
    
    pub mod guids {
        use windows::{Win32::{System::Com::StructuredStorage::IPropertyBag, Media::DirectShow::{IBaseFilter, IAMCameraControl}}, core::{ComInterface, GUID}};
        pub const P_BAG: GUID = IPropertyBag::IID;
        pub const BF: GUID = IBaseFilter::IID;
        pub const IAMCC: GUID = IAMCameraControl::IID;
    }
}


// LINKS
// https://www.appsloveworld.com/cplus/100/66/how-to-get-a-list-of-video-capture-devices-web-cameras-on-windows-c
// https://learn.microsoft.com/de-de/windows/win32/directshow/using-the-system-device-enumerator
// https://learn.microsoft.com/de-de/windows/win32/directshow/using-the-filter-mapper
// https://github.dev/cureos/aforge/blob/master/Sources/Video.DirectShow/VideoCaptureDevice.cs
