use axum::{
    routing::{get},
    Router,
};

use teloxide::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {

    // create config struct

    run_bot(token);

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn run_bot(token: String) {

    let bot = Bot::new(token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}

async fn root() -> &'static str {
    "Hello, World!"
}
