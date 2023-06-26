use async_imap::Client;
use futures_util::TryStreamExt;
use std::env::var;
use std::str::FromStr;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let server = var("SERVER")?;
    let port = var("PORT")?;
    let login = var("LOGIN")?;
    let password = var("PASSWORD")?;

    let tcp = TcpStream::connect((server.as_str(), u16::from_str(&port)?)).await?;
    let tls = async_native_tls::TlsConnector::new();
    let tls_stream = tls.connect(&server, tcp).await?;

    let client = Client::new(tls_stream);

    let mut session = match client.login(&login, &password).await {
        Ok(session) => session,
        Err((err, _)) => return Err(err.into()),
    };

    {
        session.select("INBOX").await?;

        let mut messages = session.fetch("1", "RFC822").await?;

        if let Ok(Some(m)) = messages.try_next().await {
            if let Some(body) = m.body().and_then(|b| std::str::from_utf8(b).ok()) {
                println!("{body}");
            }
        }
    }

    session.logout().await?;

    Ok(())
}
