use azalea::{prelude::*, protocol::ServerAddress};

#[tokio::main]
async fn main() {
    let account = Account::offline("itzbot");
    let client = ClientBuilder::new()
    .set_state(BotComponent::default())
    .start(account, ServerAddress::from("in1.svrx.top:27674"))
    .await.unwrap();

}

#[derive(Debug, Default, Clone, Component)]
pub struct BotComponent {
}

async fn handle(bot: Client, event: Event, state: BotComponent) -> anyhow::Result<()> {

}
