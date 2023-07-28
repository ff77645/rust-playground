


fn main(){
    let str = "罼ang";
    
    dbg!(str.as_bytes());
    dbg!(str.as_ptr());
    dbg!(str.as_ptr());
    dbg!(str.len());
    let bytes = str.bytes();
    for byte in bytes {
        print!("{byte}");
    }
    println!();
    for (index,char) in str.char_indices(){
        print!("{index} is {char}");
    }
    println!();
    for char in str.chars(){
        print!("{char}");
    }
    println!();

    let end_txt = "hello world";
    dbg!(end_txt.ends_with("world"));
    dbg!(str::ends_with(end_txt, "world"));

    let mut str = String::from("Hello world!");
    dbg!(str.get(6..));
    // dbg!(str);

    let sub_str = str.get_mut(6..8);
    let res = sub_str.map(|i|{
        i.make_ascii_uppercase();
    });
    dbg!(str);

}
