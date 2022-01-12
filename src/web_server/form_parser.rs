/// this file used to parse post requst of multipart/form-data
use actix_multipart::{Field, MultipartError, };
use futures_util::TryStreamExt;
/// this method take only one paramter which can be get bu using trynext on variable of  multipart which recived from the post request ,
/// return string that repesnt tha data
pub async fn get_string(payload:Result<Option<Field>,MultipartError>)->Option<String>
{
    // get the bytes out of the payload parameter using get_bytes 
    let string_bytes=match  get_bytes(payload).await 
    {
        Some(data)=> data,
        None=>{return None;}
    };   
    //convert bytes into string and return it into the used 
    let data = format!("{}", String::from_utf8_lossy(&string_bytes));
    Some(data)
}
/// this method take only one paramter which can be get bu using trynext on variable of  multipart which recived from the post request ,
/// return the payload data 
pub async fn get_bytes(field:Result<Option<Field>,MultipartError>)->Option<Vec<u8>>
{
    // here we extract the value field out of the paramter
    let field = match field
    {
        Ok(data) =>data,        
        Err(_) => 
        {
            println!("Fail to Parse Payload First Result Field in the form {}",line!() );
            return None;
        }
    };
    //we call get_field_bytes to get the bytes
    get_field_bytes(field).await  
}
/// a sub method that belong to get_bytes which responsible to get that post bytes
async fn get_field_bytes(field:Option<Field>)->Option<Vec<u8>>
{
    // we extract the field out of Option<field>
    let mut field = match field
    {
        Some(data) => data,
        
        None =>
        {
            println!("Fail to Parse Payload  First Option Field in the form {}",line!() );
            return None;
        }
    };
    //creating empty vector that will recive the data
    let mut bytes_vec=Vec::<u8>::new();
    // use itteration_count to check if there any buffer readed 
    let mut itteration_count=0;
    // try_next on variable of type field will convert it into bytes and the retun type is of Option<Bytes>
    // we use loop cause if the buffer is large the try next needed to be called more than one time
    while let Ok(data) = field.try_next().await 
    {
        // we extract the bytes
        let field=match data  
        {
            Some(data) =>data,
            None => 
            { 
                // if there is an iteration that happen before then break the loop abd go to returing the value
                // of no itteration did happen then that's mean that no value had been readed out of Field paramter so  return None as Result
                if itteration_count>0 
                {                    
                    break
                }      
                println!("Fail to Parse Payload First Result Field in the form {}",line!() );   
                return  None;   
            }
        };
        // increase the itteration_count every loop
        itteration_count+=1;
        // apend the bytes into the vec so we will have by the end vec that contain all the bytes
        bytes_vec.append(&mut field.to_vec());
    }
// return the vector
   Some(bytes_vec)
}
