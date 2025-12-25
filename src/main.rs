use axum::{
    routing::{get},
    Router,
};

use teloxide::prelude::*;

async fn get_coordinates() {
    // Placeholder for future implementation
}

fn create_app() -> Router {
    Router::new()
        .route("/health", get(|| async {}))
        .route("/coordinates", get(get_coordinates))
}

#[tokio::main]
async fn main() {

    // create config struct

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("failed to bind tcp listener");

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("failed to start server");
}

async fn run_bot(token: String) {

    let bot = Bot::new(token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
