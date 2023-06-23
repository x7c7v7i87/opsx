pub mod http;
pub mod payload;
pub mod hmac;
pub mod ps;
pub mod json;
pub mod gitlab;




#[test]
fn is_work() {
    assert_eq!(1, 1);
}


#[test]
fn is_json(){
    use std::fs::File;
    use std::io::Read;
    // use serde_json::Value;
    use std::path::Path;

    let path = Path::new("./test.json");
    let mut file = File::open(&path).expect("Unable to open file");
    
    // 读取文件内容
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");


    let payload = json::PayloadJson::from_json_vec(&contents.as_str());

    match payload {
        Ok(payload) => {
            for v in payload  {
                println!("payload: {:?}", v.name);
                println!("payload: {:?}", v.security_key);    
                println!("payload: {:?}", v.git_type);    
                println!("payload: {:?}", v.git_branch);    
                println!("payload: {:?}", v.git_url);
                println!("payload: {:?}", v.ext_script);    
                print!("-------------------\n");
            }
        }
        Err(e) => {
            println!("error: {:?}", e);
        }    
    }

}