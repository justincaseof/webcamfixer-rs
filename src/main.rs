use windows::{Devices::Enumeration::{DeviceInformation, DeviceWatcher}, Foundation::TypedEventHandler, Win32::{System::Com::{IEnumMoniker, CoInitializeEx, COINIT_MULTITHREADED, CLSCTX_LOCAL_SERVER, CoCreateInstance, CLSCTX_ALL, IMoniker}, Media::{DirectShow::{IFilterMapper2, ICreateDevEnum}, MediaFoundation::{CLSID_SystemDeviceEnum, CLSID_VideoInputDeviceCategory, GUID_NativeDeviceService, CLSID_VideoRenderer, CLSID_AudioProperties}}, self}, core::GUID};


fn main() {
    windows_1();
}

fn windows_1() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).expect("initialization");

        let enum_dev: ICreateDevEnum =
            CoCreateInstance(&CLSID_SystemDeviceEnum, None, CLSCTX_ALL).expect("devEnum");
        println!("enum_dev: \n\t{:?}", enum_dev);

        let mut class_enumerator: Option<IEnumMoniker> = None;
        enum_dev.CreateClassEnumerator(&CLSID_VideoInputDeviceCategory, &mut class_enumerator, 0).expect("CreateClassEnumerator");
        println!("class_enumerator: \n\t{:?}", class_enumerator);

        if class_enumerator.is_none() {
            println!("no devices in category");
            return;
        }
        let enum_moniker = class_enumerator.unwrap();
        
        // FIXME: reset?
        // let rreset = enum_moniker.Reset();
        // println!("rreset: \n\t{:?}", rreset);
        let mut monikers: Vec<Option<IMoniker>> = Vec::new();
        monikers.push(None);

        let mut lala: Option<*mut u32> = None;
        for n in 1..10 {
            let hr = enum_moniker.Next(&mut monikers, lala);
            println!("hr: \n\t{:?}", hr);
            println!("lala: \n\t{:?}", lala);
            
            let zero = monikers.get(0);
            println!("[0]: \n\t{:?}", zero);

            if lala.is_some() || zero.is_none() {
                println!("no results!");
                break;
            }
        }
    }
}

// fn windows_2() {
//     /// !!!!
//     // https://github.com/microsoft/windows-rs/issues/480
//     let mut enumerator: Option<IMMDeviceEnumerator> = None;
//     let hr = unsafe {
//         CoCreateInstance(
//             &MMDeviceEnumerator,
//             None,
//             (CLSCTX::CLSCTX_INPROC_SERVER.0 | CLSCTX::CLSCTX_LOCAL_SERVER.0 | CLSCTX::CLSCTX_REMOTE_SERVER.0) as u32,
//             &IMMDeviceEnumerator::IID,
//             &mut enumerator as *mut _ as _)
//     };
//     println!("CoCreateInstance: {:?}", hr);

//     if let Some(enumerator) = enumerator {

//         let mut devices = None;
//         let hr = enumerator.EnumAudioEndpoints(EDataFlow::eAll, DEVICE_STATE_ACTIVE, &mut devices);
//         println!("EnumAudioEndpoints: {:?}", hr);

//         if let Some(devices) = devices {
//             let mut count = 0;
//             let hr = devices.GetCount(&mut count);
//             println!("GetCount: {:?}", hr);

//             println!("Found {:?} active devices.", count);
//         }
//     }
// }

fn windows_rs_1() {
    println!("Hello, world!");
    example();
    
    // unsafe {
    //     CoInitializeEx(None, COINIT_SPEED_OVER_MEMORY).expect("lala");

    // }

    let filter = Win32::Media::MediaFoundation::CLSID_VideoInputDeviceCategory;
    let cam: Win32::Media::DirectShow::IAMCameraControl;
    let info: windows::Win32::Media::DirectShow::IFilterInfo;

    //cam.Get(property, lvalue, flags);
    let foo: windows::Win32::Media::DirectShow::ICameraControl;

    
    let imf: windows::Win32::Media::DirectShow::IMediaFilter;
    
    let res: String;
    // CLSID_SystemDeviceEnum
    // let x = CoCreateInstance(Win32::Media::MediaFoundation::CLSID_VideoInputDeviceCategory, res, CLSCTX_INPROC_SERVER);

    let pEnumCat: IEnumMoniker;
    let pMapper: IFilterMapper2;
    let enumDev: ICreateDevEnum;
    let guid_CLSID_SystemDeviceEnum = GUID::from(CLSID_SystemDeviceEnum);
    let guid_CLSID_VideoInputDeviceCategory = GUID::from(CLSID_VideoInputDeviceCategory);

    // let _enum = CoCreateInstance(CLSID_SystemDeviceEnum, enumDev, CLSCTX_INPROC_SERVER);
    // let f = ICreateDevEnum::CreateClassEnumerator(&self, clsiddeviceclass, ppenummoniker, dwflags);
    
    // let c = unsafe { CreateClassMoniker(rclsid) };
    
    // let d = CreateClassEnumerator();


    // example see https://rodrigocfd.github.io/winsafe/winsafe/fn.CoCreateInstance.html
    // let obj = unsafe {
    //     const _FOO: &String = &";".to_string();
    //     const _BAR: *mut GUID = CLSID_SystemDeviceEnum;


    //     CoCreateInstance::<ICreateDevEnum>(
    //         _BAR,
    //         None,
    //         CLSCTX_INPROC_SERVER,
    //     ).expect("...")
    // };


}

fn example() {
    loop {
        let watcher = DeviceInformation::CreateWatcher().expect("...");

        watcher.Added(&TypedEventHandler::<DeviceWatcher, DeviceInformation>::new(
            |_, info| {
                if info.as_ref().expect("info").Name().is_ok_and(|name| name.eq("HP HD Camera")) {
                    println!("");
                    println!("{}", info.as_ref().expect("info").Name()?);
                    println!("\t- {}", info.as_ref().expect("info").Id()?);
                    println!("\t- {:?}", info.as_ref().expect("info").Kind()?);
                    println!("\t- {:?}", info.as_ref().expect("info").IsEnabled()?);
                    println!("\t- {:?}", info.as_ref().expect("info").Properties()?);
                }
                Ok(())
            },
        )).expect("...");

        watcher.EnumerationCompleted(&TypedEventHandler::new(|_, _| {
            println!("done!");
            Ok(())
        })).expect("...");

        watcher.Start().expect("...");
        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}

// direct show filter overview:
// https://learn.microsoft.com/en-us/windows/win32/directshow/filter-categories#directshow-filter-categories
// --> CLSID_VideoInputDeviceCategory

// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/DirectShow/struct.ICameraControl.html

// get stuff
// https://github.com/Azure/Accord-NET/blob/master/Sources/Accord.Video.DirectShow/FilterInfoCollection.cs

// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/DirectShow/struct.IMediaFilter.html