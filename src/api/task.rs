use actix_web::{delete, get, web, Responder};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Redis {
    id: i32,
    key: String,
    value: String,
}

#[derive(Serialize)]
pub struct Response {
    success: bool,
    message: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RedisPayload {
    key: String,
    value: String,
}

#[get("/")]
pub async fn index() -> String {
    format!("string")
}
#[get("/health")]
pub async fn health() -> String {
    format!("string")
}
#[get("/get/{key}")]
pub async fn get(key: web::Path<String>) -> impl Responder {
    let r = get_value(key.to_string()).unwrap();
    return web::Json(r);
}
#[get("/get-all")]
pub async fn get_all() -> impl Responder {
    let r = get_all_values().unwrap();
    return web::Json(r);
}
fn get_all_values() -> Result<Vec<Redis>, rusqlite::Error> {
    let conn = establish_connection().expect("Error");
    let mut stmt = conn.prepare("SELECT * FROM redis;")?;
    let redis_data: Result<Vec<Redis>, _> = stmt
        .query_map([], |row| {
            Ok(Redis {
                id: row.get(0)?,
                key: row.get(1)?,
                value: row.get(2)?,
            })
        })?
        .collect();

    match redis_data {
        Ok(rows) => {
            return Ok(rows);
            //for redis in rows {
            //println!("id: {} and value: {}", redis.id, redis.value);
            //}
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(vec![Redis {
        id: 1,
        key: "".to_string(),
        value: "".to_string(),
    }])
}
pub async fn set(redis: web::Json<RedisPayload>) -> impl Responder {
    let r = get_value(redis.key.to_string()).unwrap();
    if r.id != 0 {
        return web::Json(Response {
            success: false,
            message: "key already set".to_string(),
        });
    }

    set_value(redis.key.to_string(), redis.value.to_string()).expect("error");
    return web::Json(Response {
        success: true,
        message: "success".to_string(),
    });
}
#[delete("/delete/{key}")]
pub async fn delete(key: web::Path<String>) -> impl Responder {
    delete_value(key.to_string()).expect("error");
    return web::Json(Response {
        success: true,
        message: "success".to_string(),
    });
}
fn delete_value(key: String) -> Result<()> {
    let conn = establish_connection()?;
    conn.execute("DELETE FROM redis WHERE key=:key;", &[(":key", &key)])?;
    Ok(())
}
fn set_value(key: String, value: String) -> Result<()> {
    let conn = establish_connection()?;
    let _ = conn
        .execute(
            "INSERT INTO redis(key,value) values (?1,?2)",
            [&key, &value],
        )
        .expect("Insert");

    Ok(())
}
fn get_value(key: String) -> Result<Redis, rusqlite::Error> {
    let conn = establish_connection().expect("Error");
    let mut stmt = conn.prepare("SELECT id, key, value FROM redis WHERE key=:key;")?;
    let redis_iter = stmt.query_map(&[(":key", key.to_string().as_str())], |row| {
        Ok(Redis {
            id: row.get(0)?,
            key: row.get(1)?,
            value: row.get(2)?,
        })
    })?;
    for r in redis_iter {
        //println!("Found redis {:?}", r.as_ref().unwrap());
        return r;
    }
    Ok(Redis {
        id: 0,
        key: "".to_string(),
        value: "".to_string(),
    })
}
fn establish_connection() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("redis.db");
    conn
}
