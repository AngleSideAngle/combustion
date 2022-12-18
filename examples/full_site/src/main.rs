use rocket::futures::future::ok;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    combustion::start(".").await;
    
    Ok(())
}
