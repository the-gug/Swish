pub const CERT_BUNDLE: &'static[u8] = include_bytes!("../caroot/cacert.pem"); 

pub fn get_cert_bundle() -> String {
    String::from_utf8(CERT_BUNDLE.to_vec()).unwrap()
}