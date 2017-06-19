extern crate usbids;


fn main() {
    let usbIDs = usbids::USBIDs::new();

    if let Some(res) = usbIDs.get_usb_id(1003,8257){
        println!("{} : {}", res.Vendor, res.Product);
    }
}