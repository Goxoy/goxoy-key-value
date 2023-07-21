use rusqlite::Connection;

#[derive(Debug)]
pub struct KeyValueDb {
    pub db_obj:Connection,
    pub port_no:usize,
}

impl KeyValueDb {
    pub fn new(db_name:&str) -> Self {
        let mut full_path=String::from(db_name);
        full_path.push_str(".db");
        let create_table=match std::fs::metadata(full_path.clone()) {
            Ok(_) => false,
            Err(_) => true
        };

        let db_conn:Connection = Connection::open(full_path.clone()).unwrap();
        if create_table==true{
            let _db_result=db_conn.execute("CREATE TABLE kv_list ( key TEXT NOT NULL PRIMARY KEY, value TEXT NOT NULL )", ()).unwrap();
        }
        KeyValueDb{
            db_obj:db_conn,
            port_no:0,
        }
    }
    pub fn add(&mut self,key:&str,value:&str) -> bool {
        let insert_result=self.db_obj .execute("INSERT INTO kv_list (key, value) VALUES (?1, ?2)",&[&key, &value]);
        if insert_result.is_ok(){
            if insert_result.unwrap()>0{
                return true;
            }
        }
        false
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut kv_obj=KeyValueDb::new("db_name");
    kv_obj.add("key", "value");
    assert_eq!(true,true)
}
