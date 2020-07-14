extern crate colored;
use colored::*;


pub fn test () {

    let string = r"dgdfgdfgrertert4\";
    println!("{}", string);

    let my_vector = vec!["\\u{007B}"];
    for g in my_vector {
        println!("{}", g);
    }

    let mut _utf_vec2: Vec<u8> = Vec::new();

    _utf_vec2.push(27);
    _utf_vec2.push(91);
    _utf_vec2.push(53);
    _utf_vec2.push(105);

    let _to_read = String::from_utf8(_utf_vec2.clone()).unwrap();
    print!("{} ", _to_read.red());

    let s = [27, 91, 53, 105];
    print!("{}", str::from_utf8(&s).unwrap());


    let mut _utf_vec: Vec<u8> = Vec::new();

    for i in 195..=223 {

        println!("_______________{} 128..191_______________", i);
        _utf_vec.push(i);

        for j in 128..=191 {
            _utf_vec.push(j);
            let _to_read = String::from_utf8(_utf_vec.clone()).unwrap();
            print!("{} ", _to_read.yellow());
            _utf_vec.pop();
        }

        _utf_vec.pop();
        println!();
    }
}

//assert_eq!("💖", toRead);
//240 159 146 150 //sparkleheart
//208 128
//utfVec.push(27);
//utfVec.push(91);
//utfVec.push(53);
//utfVec.push(105);
//for i in 128..191 {
    