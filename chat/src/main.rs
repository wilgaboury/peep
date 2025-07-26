use bootstrap_client::{BootstrapClient, Security};
use peep::{PeepClient, PeepClientConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bootstrap = BootstrapClient::new("18.216.61.107:80".to_string(), Security::Insecure).await?;
    let peep = PeepClient::new(PeepClientConfig {
        bootstrap,
        session: None,
    }).await?;

    println!("success: {}", peep.session);

    Ok(())
}
