use std::collections::HashMap;
use rusqlite::Connection;

#[derive(Debug)]
pub struct KeyValueDb {
    pub list:HashMap<String,String>,
    pub db_open:bool,
    pub record_loaded:bool,
    pub db_obj:Connection,
    pub db_path:String,
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
            list:HashMap::new(),
            db_open:true,
            record_loaded:false,
            db_obj:db_conn,
            db_path:full_path.clone(),
        }
    }
    pub fn load_records(&mut self){
        if self.db_open==true && self.record_loaded==false {
            let stmt = self.db_obj.prepare("SELECT * FROM kv_list");
            if stmt.is_ok(){
                let mut stmt=stmt.unwrap();
                stmt.query_map(
                    (), 
                    |row| {
                        let key_result:String=row.get(0).unwrap();
                        let value_result:String=row.get(1).unwrap();
                        Ok((key_result,value_result))
                    }
                ).into_iter().for_each(|person| {
                    person.for_each(|s|{
                        let (key_result,value_result)=s.unwrap();
                        self.list.insert(key_result, value_result);
                    });
                });
            }
            self.record_loaded=true;
        }
    }
    
    pub fn set_value(&mut self,key:&str,value:&str) -> bool {
        if self.db_open==false {
            return false;
        }
        self.load_records();
        if self.list.contains_key(key){
            let update_result=self.db_obj .execute("UPDATE kv_list SET value=?1 WHERE key=?2",&[&value, &key]);
            if update_result.is_ok(){
                if update_result.unwrap()>0{
                    self.list.insert(String::from(key),String::from(value));
                    return true;
                }
            }
        }else{
            let insert_result=self.db_obj .execute("INSERT INTO kv_list (key, value) VALUES (?1, ?2)",&[&key, &value]);
            if insert_result.is_ok(){
                if insert_result.unwrap()>0{
                    self.list.insert(String::from(key),String::from(value));
                    return true;
                }
            }
        }
        false
    }
    pub fn get_value(&mut self,key:&str) -> Option<String> {
        if self.list.contains_key(key){
            let value_str=self.list.get(key);
            Some(String::from(value_str.unwrap()))
        }else{
            None
        }
    }

    pub fn delete(&mut self,key:&str) ->bool {
        if self.db_open==false {
            return false;
        }
        self.load_records();
        let insert_result=self.db_obj .execute("DELETE FROM kv_list WHERE key = ?1",&[&key]);
        if insert_result.is_ok(){
            if insert_result.unwrap()>0{
                self.list.remove(&key.to_string());
                return true;
            }
        }
        false
    }

    pub fn flush(&mut self) -> bool{
        self.close();
        if std::fs::metadata(&self.db_path).is_ok(){
            if std::fs::remove_file(&self.db_path).is_ok(){
                return true;
            }
        }
        false
    }
    pub fn close(&mut self){
        if self.db_open==true {
            self.db_obj=Connection::open_in_memory().unwrap();
            self.db_open=false;
            self.list.clear();
            self.record_loaded=false;
        }
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let since_the_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    if since_the_epoch.is_ok(){
        let tmp_db_name=since_the_epoch.unwrap().as_micros().to_string();
        let mut kv_obj=KeyValueDb::new(&tmp_db_name);
        kv_obj.set_value("tmp_key", "tmp_value");
        let mut record_founded=false;
        let tmp_value=kv_obj.get_value("tmp_key");
        if tmp_value.is_some(){   
            if tmp_value.unwrap().eq("tmp_value"){
                record_founded=true;
            }
        }
        kv_obj.flush();
        if record_founded==true{
            assert!(true)
        }else{
            assert!(false)
        }
    }else{
        assert!(false)
    }
}
