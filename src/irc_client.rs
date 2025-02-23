use irc::client::prelude::*;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::TryStreamExt;
use crate::app::App;

pub async fn connect_to_server(app: Arc<Mutex<App>>) -> irc::error::Result<()> {
    let config = {
        let app_lock = app.lock().await;
        Config {
            nickname: Some(app_lock.nickname.clone()),
            server: Some(app_lock.hostname.clone()),
            channels: vec![app_lock.channel.clone()],
            password: Some(app_lock.password.clone()),
            ..Config::default()
        }
    };

    let client = Client::from_config(config).await?;
    client.identify()?;

    // Create channel for message passing
    let (_tx, mut rx) = mpsc::channel::<String>(100);

    // Clone for stream handler
    let app_clone = app.clone();
    let client = Arc::new(Mutex::new(client));
    let client_clone = client.clone();

    // Handle incoming messages
    tokio::spawn(async move {
        let mut stream = client_clone.lock().await.stream()?;
        while let Some(message) = stream.try_next().await? {
            match message.command {
                Command::PRIVMSG(channel, content) => {
                    let app_lock = app_clone.lock().await;
                    if channel == app_lock.channel {
                        let sender = message.prefix.map(|p| p.to_string()).unwrap_or_default();
                        drop(app_lock);
                        let mut app = app_clone.lock().await;
                        app.messages.push(format!("<{}> {}", sender, content));
                    }
                }
                Command::Response(Response::RPL_NAMREPLY, params) => {
                    if params.len() >= 4 {
                        let mut app = app_clone.lock().await;
                        app.users = params[3].split_whitespace().map(String::from).collect();
                    }
                }
                _ => {}
            }
        }
        Ok::<_, irc::error::Error>(())
    });

    // Handle sending messages
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let app_lock = app.lock().await;
            client.lock().await.send_privmsg(&app_lock.channel, &message)?;
        }
        Ok::<_, irc::error::Error>(())
    });

    Ok(())
} 