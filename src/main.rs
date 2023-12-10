use std::{mem, thread, time};

use windows::{
    core::{ComInterface, Interface, GUID, PCWSTR},
    Win32::{
        Media::{
            DirectShow::{IAMCameraControl, IBaseFilter, ICreateDevEnum, CameraControl_Focus, CameraControl_Zoom, CameraControl_Roll, CameraControl_Flags_Manual, CameraControlFlags},
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

        
        // Get ICreateDevEnum instance
        // 
        let enum_dev: ICreateDevEnum =
            CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_ALL).expect("devEnum"); // works
        println!("enum_dev: {:?}", enum_dev);

        // Query for VideoInputDevices
        // 
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

        
        // walk through results, but maximum 10 items...
        // ...and print out their stuff
        // 
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
                    let hr = m.BindToStorage(None, None, &P_BAG, &mut result__);
                    println!(" --> IPropertyBag as Storage: {:?}", hr);
                    let bag: IPropertyBag = IPropertyBag::from_raw(result__.as_mut().unwrap());
                    println!("   - bag: {:?}", bag);

                    // ######### PropertyBag->VARIANT ##################################################################################################################################
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
                    // // no idea if required...
                    // mem::forget(val);
                    // mem::forget(prop_val);
                    // mem::forget(property_name);
                    // mem::forget(pfilter);
                    // mem::drop(val);
                    // mem::drop(prop_val);
                    // mem::drop(property_name);
                    // mem::drop(pfilter);

                    // ### IBaseFilter ##########################################################################################################################################
                    let hr = m.BindToObject(None, None, &BF, &mut result__);
                    println!(" --> IBaseFilter: {:?}  --  result__: {:?}", hr, result__);
                    
                    // ###################################################################################################################################################
                    let hr = m.BindToObject(None, None, &IAMCC, &mut result__);
                    println!(" --> IAMCameraControl: {:?}  --  result__: {:?}", hr, result__);
                    if hr.is_ok() {
                        let camctl: IAMCameraControl = IAMCameraControl::from_raw(result__.as_mut().unwrap());
                        println!("   - camctl: {:?}", camctl);
                        
                        // ### READ PROPERTIES (currently not working: (Err(Error { code: HRESULT(0x80070057), message: "The parameter is incorrect." })))
                        let mut val: i32 = 0;
                        let _val = &mut val as *mut i32;

                        // let flags: *mut i32 = mem::zeroed();
                        let prop = CameraControl_Zoom.0;
                        let mut flags = CameraControlFlags::default();
                        let _flags = &mut flags.0 as *mut i32;
                        let res = camctl.Get(prop, _val, _flags);
                        println!("     - camctl.get({}): val={:?}, flags={:?} ({:?})", prop, val, *_flags, res);

                        // ### WRITE PROPERTIES
                       
                        // ROLL
                        let prop = CameraControl_Roll.0;
                        let val = 1;
                        let flags = 0;
                        let res = camctl.Set(CameraControl_Roll.0, 1, 0);  // WORKS!!!!!
                        println!("     - camctl.set({}): val={:?}, flags={:?} ({:?})", CameraControl_Roll.0, val, flags, res);
                            thread::sleep(time::Duration::from_millis(250));
                            // ROLL BACK
                            let prop = CameraControl_Roll.0;
                            let val = 1;
                            let flags = 0;
                            let res = camctl.Set(CameraControl_Roll.0, 2, 0);  // WORKS!!!!!
                            println!("     - camctl.set({}): val={:?}, flags={:?} ({:?})", CameraControl_Roll.0, val, flags, res);
                            thread::sleep(time::Duration::from_millis(250));
                            // ROLL BACK
                            let prop = CameraControl_Roll.0;
                            let val = 1;
                            let flags = 0;
                            let res = camctl.Set(CameraControl_Roll.0, 3, 0);  // WORKS!!!!!
                            println!("     - camctl.set({}): val={:?}, flags={:?} ({:?})", CameraControl_Roll.0, val, flags, res);
                            thread::sleep(time::Duration::from_millis(250));
                            // ROLL BACK
                            let prop = CameraControl_Roll.0;
                            let val = 1;
                            let flags = 0;
                            let res = camctl.Set(CameraControl_Roll.0, 0, 0);  // WORKS!!!!!
                            println!("     - camctl.set({}): val={:?}, flags={:?} ({:?})", CameraControl_Roll.0, val, flags, res);
                            thread::sleep(time::Duration::from_millis(500));

                        // ZOOM
                        let prop = CameraControl_Zoom.0;
                        let val = 1;
                        let flags = 0;
                        let res = camctl.Set(CameraControl_Zoom.0, 120, 0);  // 100 .. 400 WORKS!!!!
                        println!("     - camctl.set({}): val={:?}, flags={:?} ({:?})", CameraControl_Zoom.0, val, flags, res);
                            thread::sleep(time::Duration::from_millis(250));
                            // ZOOM BACK
                            let prop = CameraControl_Roll.0;
                            let val = 1;
                            let flags = 0;
                            let res = camctl.Set(CameraControl_Zoom.0, 100, 0);  // WORKS!!!!!
                            println!("     - camctl.set({}): val={:?}, flags={:?} ({:?})", CameraControl_Zoom.0, val, flags, res);
                        
                        // FOCUS
                        let prop = CameraControl_Focus.0;
                        let val = 1;
                        let flags = 0;
                        let res = camctl.Set(CameraControl_Focus.0, CameraControl_Flags_Manual.0, 0);  // will probably work...
                        println!("     - camctl.set({}): val={:?}, flags={:?} ({:?})", CameraControl_Focus.0, val, flags, res);
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

// these consts could be found in crate 'windows' as well. however, it was kinda hard to find them all in the first place, so i leave them here.
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
