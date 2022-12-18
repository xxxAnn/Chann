use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Serialize)]
struct BoardData {
    d: HashMap<u8, u8> 
}

//
// 0 = White Pawn
// 1 = White Pawn (Just moved)
// 2 = Black Pawn (Just moved)


#[no_mangle]
extern "C" fn analyze_position(metadata: i32, json_payload: *const c_char) -> i32 {
    let bytes = unsafe { CStr::from_ptr(json_payload).to_bytes() };
    let string = str::from_utf8(bytes).unwrap();
    let board_data: BoardData = serde_json::from_str(string).unwrap();
    let is_white = is_white(metadata);
    let white_castled = white_has_castled(metadata);
    let black_castled = black_has_castled(metadata);
    analyze_board(board_data, white_castled, black_castled, is_white, 0, 1)
}

fn analyze_board(board_data: BoardData, white_castled: bool, black_castled: bool, is_white: bool, depth: u8, max_depth: u8) -> i32 {
    todo!()
}

fn tile_number_to_tuple(n: u8) -> (u8, u8) {
    let y = n % 10;
    let x = (n - y)/10;
    (x, y)
}

fn is_white(metadata: i32) -> bool {
    metadata % 2 == 0
}

fn white_has_castled(metadata: i32) -> bool {
    let n = metadata - (metadata % 2);
    return n == 2 || n == 6
}
fn black_has_castled(metadata: i32) -> bool {
    let n = metadata - (metadata % 2);
    return n == 4 || n == 6
}