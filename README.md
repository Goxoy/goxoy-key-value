# Goxoy Key Value DB

SQLite ile key value tabanlı data kayıt işlemleri için RUST tabanlı kütüphane.
Alt kitaplık olarak "rusqlite" kitaplığı kullanılmıştır.


## Kullanım / Örnekler

```rust
let db_name = "key-value-db-name";
let mut db_obj = KeyValueDb::new(&db_name);

// kayıt ekleme ve güncelleme için
db_obj.set_value("key-text", "value-text");

// kayıt silmek için
db_obj.delete("key-text");

// kayıt okumak için
let value_obj : Option<String> = db_obj.get_value("key-text");
if value_obj.is_some(){
    println!("bulunan kayit: {}", value_obj.unwrap());
}else{
    println!("kayit bulunamadi");
}

```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)
