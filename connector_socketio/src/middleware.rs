use color_eyre::eyre::{ContextCompat, Result};
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

pub fn extract_token(s: &SocketRef) -> Result<String> {
    let token = s.extensions.get::<String>();

    token.wrap_err("Token not found in socket extensions")
}
