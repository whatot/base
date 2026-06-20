use std::time::Duration;

use kameo::{
    Actor,
    actor::Spawn,
    message::{Context, Message},
};

#[derive(Actor)]
pub struct HelloWorldActor;

pub struct Greet(String);

impl Message<Greet> for HelloWorldActor {
    type Reply = ();

    async fn handle(
        &mut self,
        Greet(str): Greet,
        _: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        println!("{str}");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let actor_ref = HelloWorldActor::spawn(HelloWorldActor);

    actor_ref
        .tell(Greet("Tell Hello world!".to_string()))
        .await?;

    actor_ref
        .ask(Greet("Ask Hello world!".to_string()))
        .reply_timeout(Duration::from_secs(1))
        .await?;

    Ok(())
}
