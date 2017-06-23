extern crate regex;

use regex::Regex;
use std::collections::HashMap;

pub struct USBIDs {
    ids: HashMap<u16, Vendor>,
}

struct Vendor {
    name: String,
    products: HashMap<u16, Product>,
}

struct Product {
    name: String,
}

pub struct USBResult {
    pub vendor: String,
    pub product: String,
}

impl Vendor {
    fn new(name: &str) -> Vendor {
        Vendor {
            name: name.to_string(),
            products: HashMap::new(),
        }
    }
}

impl Product {
    fn new(name: &str) -> Product {
        Product { name: name.to_string() }
    }
}

impl USBIDs {
    pub fn new() -> Self {
        let vendor_regex = Regex::new(r"^([[:xdigit:]]{4})  (.+)$").unwrap();
        let product_regex = Regex::new(r"^	([[:xdigit:]]{4})  (.+)$").unwrap();

        let mut vendors: HashMap<u16, Vendor> = HashMap::new();

        let mut vendor_id: u16 = 0;

        let source = include_str!("usb.ids").lines();

        for line in source {
            if !line.is_empty() {
                if let Some(vendor) = vendor_regex.captures(line) {
                    let id: u16 = vendor
                        .get(1)
                        .map(|m| u16::from_str_radix(m.as_str(), 16))
                        .unwrap()
                        .unwrap();

                    let name = vendor.get(2).map(|m| m.as_str()).unwrap();

                    let temp_vendor = Vendor::new(name);
                    vendor_id = id;

                    vendors.insert(id, temp_vendor);
                } else if let Some(product) = product_regex.captures(line) {
                    let id: u16 = product
                        .get(1)
                        .map(|m| u16::from_str_radix(m.as_str(), 16))
                        .unwrap()
                        .unwrap();

                    let name = product.get(2).map(|m| m.as_str()).unwrap();

                    if let Some(v) = vendors.get_mut(&vendor_id) {
                        let product = Product::new(name);
                        v.products.insert(id, product);
                    }
                }
            }
        }
        USBIDs { ids: vendors }
    }

    pub fn get_usb_id(&self, vendor_id: u16, product_id: u16) -> Option<USBResult> {
        if let Some(v) = self.ids.get(&vendor_id) {
            if let Some(p) = v.products.get(&product_id) {
                return Some(USBResult {
                                vendor: v.name.clone(),
                                product: p.name.clone(),
                            });
            } else {
                return Some(USBResult {
                                vendor: v.name.clone(),
                                product: String::from("<UNKNOWN_PRODUCT>"),
                            });
            }
        }

        None
    }

    pub fn get_vendor_name(&self, vendor_id: u16) -> Option<String> {
        if let Some(v) = self.ids.get(&vendor_id) {
            return Some(v.name.clone());
        }

        None
    }

    pub fn get_product_name(&self, vendor_id: u16, product_id: u16) -> Option<String> {
        if let Some(v) = self.ids.get(&vendor_id) {
            if let Some(p) = v.products.get(&product_id) {
                return Some(p.name.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
