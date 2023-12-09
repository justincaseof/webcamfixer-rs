use windows::{Devices::Enumeration::{DeviceInformation, DeviceWatcher}, Foundation::TypedEventHandler, Win32::{System::Com::{IEnumMoniker, CoInitializeEx, COINIT_MULTITHREADED, CLSCTX_LOCAL_SERVER, CoCreateInstance, CLSCTX_ALL, IMoniker}, Media::{DirectShow::{IFilterMapper2, ICreateDevEnum}, MediaFoundation::{CLSID_SystemDeviceEnum, CLSID_VideoInputDeviceCategory, GUID_NativeDeviceService, CLSID_VideoRenderer, CLSID_AudioProperties}}, self}, core::GUID};


fn main() {
    windows_1();
}

fn windows_1() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).expect("initialization");

        let enum_dev: ICreateDevEnum =
            CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_ALL).expect("devEnum");
        println!("enum_dev: {:?}", enum_dev);

        let mut class_enumerator: Option<IEnumMoniker> = None;
        enum_dev.CreateClassEnumerator(&CLSID_VideoInputDeviceCategory, &mut class_enumerator, 0).expect("CreateClassEnumerator");
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
                },
                1 => {
                    println!("end.");
                    return;
                },
                _ => {
                    println!("error: {hr}");
                    return;
                }
            }
        }
    }
}
