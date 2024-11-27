//! Program to download messages and search a word written in different combinations in those
//! messages. 
//!
//! The `TG_ID` and `TG_HASH` environment variables must be set (learn how to do it for
//! [Windows](https://ss64.com/nt/set.html) or [Linux](https://ss64.com/bash/export.html))
//! to Telegram's API ID and API hash respectively.
//!
//!
use std::io::{BufRead, Write};
use std::path::Path;
use std::time::Instant;
use std::{env, fs, io};

use grammers_client::{Client, Config, SignInError};
use simple_logger::SimpleLogger;
use tokio::runtime;

use grammers_client::session::Session;

use tg_word_search::count_word;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const SESSION_FILE: &str = "downloader.session";

async fn async_main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let api_id = env!("TG_ID").parse().expect("TG_ID invalid");
    let api_hash = env!("TG_HASH").to_string();
    let chat_name = env::args().nth(1).expect("chat name missing");
    let word_to_search = env::args().nth(1).expect("word to search is missing");

    println!("Connecting to Telegram...");
    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    })
    .await?;
    println!("Connected!");

    // If we can't save the session, sign out once we're done.
    let mut sign_out = false;

    if !client.is_authorized().await? {
        println!("Signing in...");
        let phone = prompt("Enter your phone number (international format): ")?;
        let token = client.request_login_code(&phone).await?;
        let code = prompt("Enter the code you received: ")?;
        let signed_in = client.sign_in(&token, &code).await;
        match signed_in {
            Err(SignInError::PasswordRequired(password_token)) => {
                // Note: this `prompt` method will echo the password in the console.
                //       Real code might want to use a better way to handle this.
                let hint = password_token.hint().unwrap();
                let prompt_message = format!("Enter the password (hint {}): ", &hint);
                let password = prompt(prompt_message.as_str())?;

                client
                    .check_password(password_token, password.trim())
                    .await?;
            }
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
        println!("Signed in!");
        match client.session().save_to_file(SESSION_FILE) {
            Ok(_) => {}
            Err(e) => {
                println!("NOTE: failed to save the session, will sign out when done: {e}");
                sign_out = true;
            }
        }
    }

    let maybe_chat = client.resolve_username(chat_name.as_str()).await?;

    let chat = maybe_chat.unwrap_or_else(|| panic!("Chat {chat_name} could not be found"));

    let mut messages = client.iter_messages(&chat);
    let logs_dest = Path::new("target/love_counts.csv");

    let total_messages = messages.total().await.unwrap();

    println!("Chat {} has {} total messages.", chat_name, total_messages);

    let mut counter = 0;
    let mut word_count = 0;

    let start = Instant::now();

    while let Some(msg) = messages.next().await? {
        counter += 1;
        let text = msg.text();

        word_count += count_word(text, &word_to_search);

        if counter % 50 == 0 {
            fs::write(logs_dest, format!("{counter},{word_count}"))?;
            let done_pc = counter * 100 / total_messages;
            println!("{done_pc}% messages scanned.");
            let elapsed = start.elapsed().as_secs();
            {
                let t = elapsed * total_messages as u64 / counter as u64 - elapsed;
                let (ss, mm, hh) = (t % 60, (t / 60) % 60, (t / 60) / 60);
                print!("Remaining ");
                if hh > 0 {
                    print!("{hh} hours,");
                }
                if mm > 0 {
                    print!("{mm} minutes,");
                }
                print!("{ss} seconds");
            }
        }
    }

    println!("Downloaded {counter} messages");
    println!("Love count: {word_count}");
    fs::write(logs_dest, format!("{counter},{word_count}"))?;

    if sign_out {
        // TODO revisit examples and get rid of "handle references" (also, this panics)
        drop(client.sign_out_disconnect().await);
    }

    Ok(())
}

fn main() -> Result<()> {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

fn prompt(message: &str) -> Result<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}
