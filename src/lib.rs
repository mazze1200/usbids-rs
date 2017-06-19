extern crate regex;

use regex::Regex;
use std::collections::HashMap;

pub struct USBIDs {
    ids: HashMap<u32, Vendor>,
}

struct Vendor {
    id: u32,
    name: String,
    products: HashMap<u32, Product>,
}

struct Product {
    id: u32,
    name: String,
}

pub struct USBResult {
    pub Vendor: String,
    pub Product: String,
}

impl Vendor {
    fn new(id: u32, name: &str) -> Vendor {
        Vendor {
            id: id,
            name: name.to_string(),
            products: HashMap::new(),
        }
    }
}

impl Product {
    fn new(id: u32, name: &str) -> Product {
        Product {
            id: id,
            name: name.to_string(),
        }
    }
}


impl USBIDs {
    pub fn new() -> Self {
                let vendor_regex = Regex::new(r"^([[:xdigit:]]{4})  (.+)$").unwrap();
                let product_regex = Regex::new(r"^	([[:xdigit:]]{4})  (.+)$").unwrap();

                let mut vendors: HashMap<u32, Vendor> = HashMap::new();

                let mut vendor_id = 0;

                let source = include_str!("usb.ids").lines();

                for line in source {
                        if !line.is_empty() {
                            if let Some(vendor) = vendor_regex.captures(line) {
                                let id = vendor
                                    .get(1)
                                    .map(|m| u32::from_str_radix(m.as_str(), 16))
                                    .unwrap()
                                    .unwrap();

                                let name = vendor.get(2).map(|m| m.as_str()).unwrap();

                                let temp_vendor = Vendor::new(id, name);
                                vendor_id = id;

                                vendors.insert(id, temp_vendor);
                            } else if let Some(product) = product_regex.captures(line) {
                                let id = product
                                    .get(1)
                                    .map(|m| u32::from_str_radix(m.as_str(), 16))
                                    .unwrap()
                                    .unwrap();

                                let name = product.get(2).map(|m| m.as_str()).unwrap();

                                if let Some(v) = vendors.get_mut(&vendor_id) {
                                    let product = Product::new(id, name);
                                    v.products.insert(id, product);
                                }
                            }
                        }
                }
                USBIDs { ids: vendors }
    }

    pub fn get_usb_id(&self, vendor_id: u32, product_id: u32) -> Option<USBResult> {
        if let Some(v) = self.ids.get(&vendor_id) {
            if let Some(p) = v.products.get(&product_id) {
                return Some(USBResult {                    
                                Vendor: v.name.clone(),
                                Product: p.name.clone(),
                            });
            }
        }

        None
    }
}

// fn main() {


//     let usbIDs = USBIDs::new();

//     if let Some(res) = usbIDs.get_usb_id(1003,8257){
//         println!("{} : {}", res.Vendor, res.Product);
//     }

// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
