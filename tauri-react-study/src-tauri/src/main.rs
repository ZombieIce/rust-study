// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time};
use serde::{Deserialize, Serialize};
use tauri::Window;
mod monitor;


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

#[tauri::command]
fn get_person(name: String, age: i32) -> Result<Person, String> {
    Ok(Person { name, age })
}

#[tauri::command]
fn hello1() {
    println!("hello-1 tauri!");
}

#[tauri::command]
fn hello2(msg: String) {
    println!("hello-2 tauri! {}", msg);
}

#[tauri::command]
fn hello3(msg: String) -> String {
    println!("hello-3 tauri! {}", msg);
    format!("hello-3 tauri! {}", msg)
}

#[tauri::command]
async fn method_1() -> String {
    println!("method_1");

    let result = method_2();

    println!("do another thing in method_1");

    let result = result.await;
    println!("method_2 result: {} from method_1", result);
    result
}

#[tauri::command]
async fn method_2() -> String {
    println!("method_2");
    thread::sleep(time::Duration::from_secs(3));
    format!("method_2 result")
}

struct DatabaseState {
    connected: bool,
}

fn connect_db() -> DatabaseState {
    DatabaseState { connected: true }
}

#[tauri::command]
fn query_data(state: tauri::State<DatabaseState>) {
    assert_eq!(state.connected, true);
    println!("query_data success");
}

static mut FLAG: i8 = 0;

#[tauri::command]
async fn init_process(window: Window) {
    use monitor::monitor;
    println!("init_process called");
    unsafe {
        if FLAG == 1 {
            println!("already emit init_process");
            return;
        }

        let handler = tokio::spawn(async move {
            loop {
                window.emit("my-event", monitor::Payload{
                    message: monitor::monitor(),
                    timestamp: monitor::timestamp(),
                }).unwrap();
                FLAG = 1;
                println!("emit:{}", monitor::timestamp().to_string());
                
                // sleep 100 milliseonds by tokio
                tokio::time::sleep(time::Duration::from_millis(100)).await;

                
                
            }
        });
        handler.await.unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .manage(connect_db())
        .invoke_handler(tauri::generate_handler![
            hello1, hello2, hello3, get_person, method_1, query_data, init_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
