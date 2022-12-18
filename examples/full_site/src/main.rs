use rocket::futures::future::ok;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    forest_fire::start(".").await;
    
    Ok(())
}
