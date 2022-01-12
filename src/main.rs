use actix_multipart::{Multipart, Field, MultipartError};
use actix_web::{
    get, post,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

use futures_util::TryStreamExt as _;
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    vec,
};

#[derive(Deserialize)]
struct FileInfo {
    path: String,
}

#[get("/GetImageAsByteArray")]
async fn get_image_as_byte_array(request: web::Query<FileInfo>) -> impl Responder {
    let file = File::open(&request.path);
    if file.is_err() {
        return HttpResponse::Ok().body(format!("File not found {}", &request.path));
    }
    let metadata = fs::metadata(&request.path);
    if metadata.is_err() {
        return HttpResponse::Ok().body("Couldnt read File metadata");
    }
    let mut buffer: Vec<u8> = vec![0; metadata.unwrap().len() as usize];
    if file.unwrap().read(&mut buffer).is_err() {
        return HttpResponse::Ok().body("Couldnt read File Data");
    }
    HttpResponse::Ok().body(buffer)
}
#[post("/SaveImage")]
async fn save_image(mut payload: Multipart) -> impl Responder {
  
    let directoy=match  get_bytes(payload.try_next().await).await {
        Some(data)=> data,
        None=> return HttpResponse::Ok().body("Fail to Field Parse  First Result Field in the form")
    };
    let directoy = format!("{}", String::from_utf8_lossy(&directoy));
    let name_bytes=match  get_bytes(payload.try_next().await).await {
        Some(data)=> data,
        None=> return HttpResponse::Ok().body("Fail to Field Parse  First Result Field in the form")
    };
   
    let file_name = format!("{}", String::from_utf8_lossy(&name_bytes));
    println!("{} {}",directoy,file_name);
    let content=match  get_bytes(payload.try_next().await).await {
        Some(data)=> data,
        None=> return HttpResponse::Ok().body("Fail to Field Parse  First Result Field in the form")
    };   
    let mut file = match web::block(move || std::fs::File::create(&file_name)).await
    {
        Ok(data) => data,
        Err(_) => 
        {
            return HttpResponse::Ok().body("Couldnt Create File");
        }
    };   
    return match  file.write_all(&content).map(|_| file)
    {
        Ok(_) => HttpResponse::Ok().body("Done"),
        Err(_) => HttpResponse::Ok().body("Failed to Copy Recived data into file"),
    }; 

}
async fn get_bytes(field:Result<Option<Field>,MultipartError>)->Option<Vec<u8>>
{
    let field = match field
    {
        Ok(data) =>data,        
        Err(_) => 
        {
            println!("Fail to Parse Payload First Result Field in the form {}",line!() );
            return None;
        }
    };
    get_field_bytes(field).await

    
    

}
async fn get_field_bytes(field:Option<Field>)->Option<Vec<u8>>
{
    let mut field = match field
    {
        Some(data) => data,
        
        None =>
        {
            println!("Fail to Parse Payload  First Option Field in the form {}",line!() );
              return None;
        }
    };
    let mut bytes_vec=Vec::<u8>::new();
    let mut itteration_count=0;
    while let Ok(data) = field.try_next().await 
    {

        let field=match data  
        {
            Some(data) =>data,
            None => 
            { 
                if itteration_count>0 {
                    
                    break
                }      
                println!("Fail to Parse Payload First Result Field in the form {}",line!() );   
                return  None;   
            }
        };
        itteration_count+=1;
        bytes_vec.append(&mut field.to_vec());
    }

   

   Some(bytes_vec)

}

#[get("/")]
fn index() -> HttpResponse {
  
    let html = include_str!("../Form.html");
    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(get_image_as_byte_array)
            .service(save_image)
            .service(index)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
