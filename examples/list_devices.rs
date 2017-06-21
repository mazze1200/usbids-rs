extern crate usbids;
extern crate libusb;

use std::time::Duration;

fn list_devices() -> libusb::Result<()> {
    let timeout = Duration::from_secs(1);

    let context = try!(libusb::Context::new());
    let usbIDs = usbids::USBIDs::new();

    for device in try!(context.devices()).iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if let Some(usb) = usbIDs.get_usb_id(device_desc.vendor_id() as u32,
                                       device_desc.product_id() as u32) {
            println!("Vendor:  {:#06x} {}", device_desc.vendor_id(), usb.vendor);
            println!("Product: {:#06x} {}", device_desc.product_id(), usb.product);
        } else {
            println!("Vendor:  {:#06x}", device_desc.vendor_id(),);
            println!("Product: {:#06x}", device_desc.product_id());
        }

        println!("-------------------");
    }

    Ok(())
}

fn main() {

    list_devices();
    // let usbIDs = usbids::USBIDs::new();

    // if let Some(res) = usbIDs.get_usb_id(1003,8257){
    //     println!("{} : {}", res.vendor, res.product);
    // }
}