use std::env;


fn main(){


    let args = env::args();
    dbg!(args);

    let current_dir = env::current_dir().unwrap();
    dbg!(current_dir);

    let current_exe = env::current_exe().unwrap();
    dbg!(current_exe);

    let home_dir = env::home_dir().unwrap();
    dbg!(home_dir);


    let key = "TEST";
    let val = "TEST_VALUE";
    // env::set_var(key, val);

    assert_eq!(env::var(key),Ok(val.to_string()));

    let temp_dit = env::temp_dir();
    dbg!(temp_dit);

    // let path = env::var("PATH").unwrap();
    // let path_values = env::split_paths(&path);
    // for path_val in path_values {
    //     println!("path_val {}",path_val.display());
    // }

    
    // let var = env::var("PATH").unwrap();
    // dbg!(var);


    // let var_os = env::var_os("PATH").unwrap();
    // dbg!(var_os);


    // let vars = env::vars();
    // for (key,value) in vars {
    //     println!("{key} => {value}");
    // }

}