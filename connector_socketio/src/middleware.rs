use color_eyre::eyre::Result;
use socketioxide::extract::SocketRef;
use tracing::error;

pub fn auth_middleware(s: SocketRef) -> Result<()> {
    if let Some(token_string) = s.req_parts().headers.get("Authorization") {
        let token = token_string.to_str()?.trim_start_matches("Organization ");

        // Storing the token in socket extensions
        s.extensions.insert(token.to_string());

        Ok(())
    } else {
        Err(color_eyre::eyre::eyre!("Unauthorized"))
    }
}

pub fn extract_token(s: &SocketRef) -> String {
    let token = s.extensions.get::<String>();

    if token.is_none() {
        error!("No token found in socket extensions");
    }

    token.unwrap()
}
