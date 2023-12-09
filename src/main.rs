use std::ptr;

use windows::{
    core::{ComInterface, Interface, GUID, PWSTR, PCWSTR},
    Devices::Enumeration::{DeviceInformation, DeviceWatcher},
    Foundation::{Collections::IPropertySet, TypedEventHandler, IPropertyValue},
    Win32::{
        self,
        Media::{
            DirectShow::{
                IAMCameraControl, IBaseFilter, ICameraControl, ICreateDevEnum, IFilterMapper2, BDA_STRING,
            },
            MediaFoundation::{
                CLSID_AudioProperties, CLSID_CameraControlPropertyPage, CLSID_FilterGraph,
                CLSID_FilterGraphNoThread, CLSID_FilterGraphPrivateThread, CLSID_FilterMapper,
                CLSID_FilterMapper2, CLSID_MediaPropertyBag, CLSID_SystemDeviceEnum,
                CLSID_VideoInputDeviceCategory, CLSID_VideoRenderer, GUID_NativeDeviceService,
            },
        },
        System::{Com::{
            CoCreateInstance, CoInitializeEx, CreateBindCtx, IEnumMoniker, IMoniker,
            MkParseDisplayName,
            StructuredStorage::{IPropertyBag, IPropertyBag2, PROPBAG2},
            CLSCTX, CLSCTX_ACTIVATE_32_BIT_SERVER, CLSCTX_ALL, CLSCTX_APPCONTAINER,
            CLSCTX_LOCAL_SERVER, CLSCTX_SERVER, COINIT_MULTITHREADED,
        }, Variant::{self, VARIANT, VT_BSTR, VARENUM, VT_LPWSTR}},
    },
};

fn main() {
    windows_1();
}

fn windows_1() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).expect("initialization");

        let enum_dev: ICreateDevEnum =
            CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_ALL).expect("devEnum"); // works
                                                                                           // CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_LOCAL_SERVER).expect("devEnum");  // doesn't work
                                                                                           // CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX(0)).expect("devEnum");  // doesn't work
                                                                                           // CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_SERVER).expect("devEnum");  // works
                                                                                           // CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_APPCONTAINER).expect("devEnum");  // doesn't work
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

        // FIXME: reset?
        // let rreset = enum_moniker.Reset();
        // println!("rreset: \{:?}", rreset);
        let mut monikers: Vec<Option<IMoniker>> = Vec::new();
        monikers.push(None);

        let mut celt: Option<*mut u32> = None;
        for n in 0..10 {
            let hr = enum_moniker.Next(&mut monikers, celt);

            match hr.0 {
                0 => {
                    // ok
                    let current = monikers.get(0).expect("current");
                    println!("[{n}] found: {:?}", current);

                    let bindCtx = CreateBindCtx(0).expect("bindctx");

                    let m = current.as_ref().unwrap();
                    println!(" --> m: {:?}", m);

                    // IPropertyBag for names and stuff
                    // let res: IPropertyBag;
                    // m.BindToObject(None, None, &IPropertyBag, _);

                    // IAMCameraControl for names and stuff
                    let mut res: IAMCameraControl;
                    let mut result__ = ::std::ptr::null_mut();

                    // let hr =
                    //     m.BindToObject(None, None, &CLSID_CameraControlPropertyPage, &mut result__);
                    // println!(
                    //     " --> CLSID_CameraControlPropertyPage: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &CLSID_MediaPropertyBag, &mut result__);
                    // println!(
                    //     " --> CLSID_MediaPropertyBag: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &CLSID_FilterGraph, &mut result__);
                    // println!(
                    //     " --> CLSID_FilterGraph: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &CLSID_FilterGraphNoThread, &mut result__);
                    // println!(
                    //     " --> CLSID_FilterGraphNoThread: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr =
                    //     m.BindToObject(None, None, &CLSID_FilterGraphPrivateThread, &mut result__);
                    // println!(
                    //     " --> CLSID_FilterGraphPrivateThread: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &CLSID_FilterMapper, &mut result__);
                    // println!(
                    //     " --> CLSID_FilterMapper: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &CLSID_FilterMapper2, &mut result__);
                    // println!(
                    //     " --> CLSID_FilterMapper2: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &CLSID_MediaPropertyBag, &mut result__);
                    // println!(
                    //     " --> CLSID_MediaPropertyBag: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &P_BAG, &mut result__);
                    // println!(
                    //     " --> IPropertyBag: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &P_BAG2, &mut result__);
                    // println!(
                    //     " --> IPropertyBag2: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let hr = m.BindToObject(None, None, &IAMCC, &mut result__);
                    // println!(
                    //     " --> IAMCC: {:?}  --  result__: {:?}",
                    //     hr, result__
                    // );

                    // let mut props1: IPropertyBag = ::std::ptr::null_mut();
                    // let mut props11: *mut *mut std::ffi::c_void = &mut props1.as_raw();
                    // let mut props2: *mut IPropertyBag;

                    let hr = m.BindToStorage(None, None, &P_BAG, &mut result__);
                    println!(" --> IPropertyBag as Storage: {:?}", hr);
                    let bag: IPropertyBag = IPropertyBag::from_raw(result__.as_mut().unwrap());
                    println!("   - bag: {:?}", bag);
                    
                    let pv: IPropertyValue;
                    let mut buf = ::std::ptr::null_mut();
                    let v: Vec<u16> = "FriendlyName".encode_utf16().collect();
                    let pws: PWSTR = PWSTR::from_raw(buf);
                    
                    // let foo: PCWSTR = std::mem::zeroed(); // ok, but invalid pointer
                    let foo: PCWSTR = PCWSTR::from_raw(v.as_ptr());

                    let mut pval: *mut IPropertyValue;
                    let mut pvar: *mut VARIANT = ::std::ptr::null_mut();
                    let mut pvar2: VARIANT = Default::default();
                    let pvar3: *mut VARIANT = &mut pvar2;

                    // let a: PROPBAG2;
                    let name: PROPBAG2 = PROPBAG2 {
                        dwType: VT_LPWSTR.0 as u32,
                        vt: VARENUM::from(VT_LPWSTR),
                        cfType: 0,
                        dwHint: 0,
                        pstrName: pws,
                        clsid: CLSID_MediaPropertyBag,
                    };
                    let read =  
                        bag.Read(foo, 
                            pvar3, 
                            None
                        );
                    println!("   - read: {:?}", read);

                    let mut filter: IBaseFilter;
                    let hr = m.BindToObject(None, None, &BF, &mut result__);
                    println!(" --> IBaseFilter: {:?}  --  result__: {:?}", hr, result__);

                    let mut iamcc: IAMCameraControl;
                    let hr = m.BindToObject(None, None, &IAMCC, &mut result__);
                    println!(" --> IAMCameraControl: {:?}  --  result__: {:?}", hr, result__);
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

pub const P_BAG: GUID = IPropertyBag::IID;
pub const BF: GUID = IBaseFilter::IID;
pub const IAMCC: GUID = IAMCameraControl::IID;

// LINKS
// https://www.appsloveworld.com/cplus/100/66/how-to-get-a-list-of-video-capture-devices-web-cameras-on-windows-c
// https://learn.microsoft.com/de-de/windows/win32/directshow/using-the-system-device-enumerator
// https://learn.microsoft.com/de-de/windows/win32/directshow/using-the-filter-mapper
// https://github.dev/cureos/aforge/blob/master/Sources/Video.DirectShow/VideoCaptureDevice.cs
