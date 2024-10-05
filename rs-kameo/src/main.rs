use std::time::Duration;

use kameo::{
    message::{Context, Message},
    request::MessageSend,
    spawn, Actor,
};

#[derive(Actor)]
#[actor(name = "hello_world_actor", mailbox = bounded)]
pub struct HelloWorldActor;

pub struct Greet(String);

impl Message<Greet> for HelloWorldActor {
    type Reply = ();

    async fn handle(
        &mut self,
        Greet(str): Greet,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        println!("{str}");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let actor_ref = spawn(HelloWorldActor);

    actor_ref
        .tell(Greet("Tell Hello world!".to_string()))
        .send()
        .await?;

    actor_ref
        .ask(Greet("Ask Hello world!".to_string()))
        .reply_timeout(Duration::from_secs(1))
        .send()
        .await?;

    Ok(())
}
