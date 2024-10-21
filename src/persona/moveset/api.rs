use arcropolis_api::arc_callback;
use arcropolis_api::hash40;
use arcropolis_api::load_original_file;

use crate::moveset::*;

#[arc_callback]
pub fn my_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    unsafe {
        for i in 0..50 {
            let hash_word = "fighter/jack/model/body/c0".to_owned() + &i.to_string() + "/kasumi.nutexb";
            if hash == hash40(&hash_word).as_u64() {
                SUMI.push(i);
            }
        }
    }
    load_original_file(hash, &mut data)
}