use std::collections::HashMap;

use rusqlite::Connection;

#[derive(Debug)]
pub struct KeyValueDb {
    pub list:HashMap<String,String>,
    pub db_open:bool,
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
        let mut tmp_list:HashMap<String,String>=HashMap::new();
        let db_conn:Connection = Connection::open(full_path.clone()).unwrap();
        if create_table==true{
            let _db_result=db_conn.execute("CREATE TABLE kv_list ( key TEXT NOT NULL PRIMARY KEY, value TEXT NOT NULL )", ()).unwrap();
        }else{
            let stmt = db_conn.prepare("SELECT value FROM kv_list WHERE key=:key");
            if stmt.is_ok(){
                let mut stmt=stmt.unwrap();
                stmt.query_map(
                    (), 
                    |row| {
                        let row_result:String=row.get(0).unwrap();
                        println!("row_result: {}",row_result);
                        Ok(row_result)
                    }
                ).into_iter().for_each(|person| {
                    person.for_each(|s|{
                        //self.list.insert(k, v)
                        //founded=true;
                        let result_str=s.unwrap();
                        println!("result_str: {}",result_str);
                    });
                });
            }
        }
        KeyValueDb{
            list:tmp_list,
            db_open:true,
            db_obj:db_conn,
            db_path:full_path.clone(),
        }
    }
    pub fn load_all(&mut self){
        self.list.clear();
       
    }
    pub fn add(&mut self,key:&str,value:&str) -> bool {
        if self.db_open==false {
            return false;
        }
        let insert_result=self.db_obj .execute("INSERT INTO kv_list (key, value) VALUES (?1, ?2)",&[&key, &value]);
        if insert_result.is_ok(){
            if insert_result.unwrap()>0{
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
        }
    }
    pub fn delete(&mut self,key:&str) ->bool {
        if self.db_open==false {
            return false;
        }
        let insert_result=self.db_obj .execute("DELETE FROM kv_list WHERE key = ?1",&[&key]);
        if insert_result.is_ok(){
            if insert_result.unwrap()>0{
                return true;
            }
        }
        false
    }
    pub fn get_value(&mut self,key:&str) -> Option<String> {
        if self.db_open==false {
            return None;
        }

        let stmt = self.db_obj.prepare("SELECT value FROM kv_list WHERE key=:key");
        let mut result_str=String::new();
        let mut founded=false;
        if stmt.is_ok(){
            let mut stmt=stmt.unwrap();
            stmt.query_map(
                &[(":key", key.to_string().as_str())], 
                |row| {
                    let row_result:String=row.get(0).unwrap();
                    Ok(row_result)
                }
            ).into_iter().for_each(|person| {
                person.for_each(|s|{
                    founded=true;
                    result_str=s.unwrap();
                });
            });            
        }
        if founded==true{
            Some(result_str)
        }else{
            None
        }
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut kv_obj=KeyValueDb::new("db_name");
    kv_obj.add("key1", "value");
    kv_obj.flush();
    //kv_obj.add("key2", "value");
    //kv_obj.add("key3", "value");
    //kv_obj.add("key4", "value");
    //kv_obj.delete("key3");
    //let rst=kv_obj.get_value("key3");
    //dbg!(rst);
    
    
    assert_eq!(true,true)
}
