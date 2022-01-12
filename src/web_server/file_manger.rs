pub mod file_manger{
    use std::{fs, io::Write};

    use actix_web::web;

   pub async fn save_file(file_name:String,folder_name:String,content:Vec<u8>)->bool
    {
        if fs::create_dir_all(&folder_name).is_err()
        {
            return false;
        }    
        let full_path=folder_name+"/"+&file_name;   
        let mut file = match web::block(move || std::fs::File::create(&full_path)).await
        {
            Ok(data) => data,
            Err(_) => 
            {
                return false;
            }
        };   
        file.write_all(&content).map(|_| file).is_ok()
    }
    }