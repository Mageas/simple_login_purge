use simple_login_rs::alias::AliasData;
use simple_login_rs::{AliasFilter, SimpleLoginClient};

use anyhow::Result;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    // let config = Config::create("test")?;
    // dbg!(config);

    let mut client = SimpleLoginClient::new("app.simplelogin.io");
    client.token = Some(&config.token);

    let aliases = get_aliases(&client).await?;
    let filtered_aliases = filter_aliases(&aliases, "Created by catchall option");

    for alias in filtered_aliases {
        dbg!(&alias);

        let response = client.alias().delete(alias.id).await?;
        dbg!(&response);

        break;
    }

    // dbg!(&filtered_aliases, filtered_aliases.len());

    Ok(())
}

/// Filter aliases by note
fn filter_aliases<'a>(aliases: &'a Vec<AliasData>, filter_note: &str) -> Vec<&'a AliasData> {
    aliases
        .iter()
        .filter(|alias| {
            if let Some(note) = &alias.note {
                note == filter_note
            } else {
                false
            }
        })
        .collect()
}

/// Get all the actives aliases
async fn get_aliases(client: &SimpleLoginClient<'_>) -> Result<Vec<AliasData>> {
    let mut return_vec: Vec<AliasData> = Vec::new();
    let mut response_len = u8::MAX;

    let mut i = 0;

    while response_len >= 20 {
        let response = client.alias().list(i, AliasFilter::Enabled).await?;
        response_len = response.len() as u8;
        return_vec = [return_vec, response].concat();
        i += 1;
    }

    Ok(return_vec)
}
