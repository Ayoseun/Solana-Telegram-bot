

#[get("/")]
pub async fn home() -> String {
    let resp = "Bot is Alive".to_string(); // Convert the string to String
    resp // Explicit return
}