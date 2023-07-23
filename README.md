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

```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)
