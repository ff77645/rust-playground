use std::{
    io::Result,
    fs,
    path::Path,
};

fn main()->Result<()>{

    // 获取目录的绝对路径
    let current_path = fs::canonicalize("./target")?;
    dbg!(current_path);

    let copy_len = fs::copy(Path::new("copy.txt"), "copy1.txt")?;
    dbg!(copy_len);

    let dir1 = "./test_create_dir";
    let dir2 = "./test/test_create_dir";
    let dir3 = "test_create_dir2";
    // fs::create_dir(dir1).unwrap();
    // fs::create_dir_all(dir2).unwrap();
    // fs::create_dir(dir3).unwrap();


    let txt1 = "./copy.txt";
    let txt2 = "./copy3.txt";
    // 不能用于文件夹
    // fs::hard_link(txt1, txt2).unwrap();

    let txt1_meta = fs::metadata(txt1).unwrap();
    dbg!(txt1_meta);
    let dir1_meta = fs::metadata(dir1).unwrap();
    dbg!(dir1_meta);


    let read_txt1 = fs::read(txt1).unwrap();
    // dbg!(read_txt1.clone());

    let txt1_str = String::from_utf8(read_txt1).unwrap();
    dbg!(txt1_str);

    let src_dir = "./src";
    let mut src_dir = fs::read_dir(src_dir).unwrap();
    // dbg!(src_dir);
    // dbg!(&src_dir.next().unwrap());
    // dbg!(&src_dir.next().unwrap());

    let path = Path::new("./src");
    let f = |entry:&fs::DirEntry| println!("enrty name: {:?}",entry.file_name());
    traverse_dir(&path,&f)?;
    Ok(())
}

fn traverse_dir(dir:&Path,cb:&dyn Fn(&fs::DirEntry))->Result<()>{
    if dir.is_dir(){
        for entry in fs::read_dir(dir)?{
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                traverse_dir(&path, cb)?;
            }else {
                cb(&entry);
            }
        }
    }
    Ok(())
}