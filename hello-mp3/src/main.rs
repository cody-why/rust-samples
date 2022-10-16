/***
 * @Author: plucky
 * @Date: 2022-09-02 23:51:02
 * @LastEditTime: 2022-09-03 00:36:37
 * @Description: 
 */

use std::io::Cursor;

fn main() {
    // download_file();
    play_online();
    
}

pub fn play_file(file: &str) {
    let file = std::fs::File::open(file).unwrap();
    let (_stream, handle)=rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let source = rodio::Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

pub fn download_file() {
    let url = "https://gr-sycdn.kuwo.cn/5cea8e8045e5f650d1bb82a784c0f532/63122e5c/resource/n3/69/83/3484564316.mp3";
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut file= std::fs::File::create("test.mp3").unwrap();
    // file.write_all (&resp.bytes().expect("Unable to read the data")).expect("Unable to write fill")；
    std::io::copy( &mut resp, &mut file).expect( "Unable to copy data");
}

pub fn play_online() {
    let url = "https://gr-sycdn.kuwo.cn/5cea8e8045e5f650d1bb82a784c0f532/63122e5c/resource/n3/69/83/3484564316.mp3";
    let resp = reqwest::blocking::get(url).unwrap();

    let cursor = Cursor::new(resp.bytes().unwrap());

    let (_stream, handle)=rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
   
    let source = rodio::Decoder::new(cursor).unwrap();
    sink.append(source);
    sink.sleep_until_end();
    
}