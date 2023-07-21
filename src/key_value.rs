#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct KeyValueDb {
    pub ip_address:String,
    pub port_no:usize,
}

impl KeyValueDb {
    pub fn new() -> Self {
        KeyValueDb{
            ip_address:String::new(),
            port_no:0,
        }
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture

    assert_eq!(true,true)
}
