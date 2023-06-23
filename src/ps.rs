use tokio::process::Command;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Params {
    pub keyword: Option<String>,
}

impl Params {
    pub fn new() -> Self {
        Params { keyword: None }
    
    }

    pub async fn cmd(cmd: String) {
        let mut child = Command::new("/bin/bash")
            .arg(cmd)
            .spawn()
            .expect("failed to spawn");

        // Await until the command completes
        let status = child.wait().await;
        let bl = match status {
            Ok(status) => {
                println!("the command exited with: {}", status);
                true
            }
            Err(e) => {
                println!("the command failed with: {}", e);
                false
            }
        };


        print!("bl: {}", bl);
    }

    
    pub async fn test() {
        println!("test is ok");
    }


}
