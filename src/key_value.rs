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
    pub fn delete(&mut self,key:&str) ->bool {
        let insert_result=self.db_obj .execute("DELETE FROM kv_list WHERE key = ?1",&[&key]);
        if insert_result.is_ok(){
            if insert_result.unwrap()>0{
                return true;
            }
        }
        false
    }
    
    pub fn get_value(&mut self,key:&str) -> bool {
        println!("key: {}",key);
        let stmt = self.db_obj.prepare("SELECT value FROM kv_list WHERE key=:key");
        if stmt.is_ok(){
            println!("stmt.is_ok()");
            let mut stmt=stmt.unwrap();
            let _person_iter = stmt.query_map(
                &[(":key", key.to_string().as_str())], 
                |row| {
                    let row_result:String=row.get(0).unwrap();
                    Ok(row_result)
                }
            );
            for person in _person_iter {
                person.for_each(move |s|{
                    let dddd=s.unwrap();
                    dbg!(dddd);
                });
            }            
        }
        return false;
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut kv_obj=KeyValueDb::new("db_name");
    //kv_obj.add("key1", "value");
    //kv_obj.add("key2", "value");
    //kv_obj.add("key3", "value");
    //kv_obj.add("key4", "value");
    //kv_obj.delete("key3");
    kv_obj.get_value("key3");
    
    
    assert_eq!(true,true)
}
