#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    system_update_manager_lib::run()
}
