use crate::utils::into_lsp_error;
use anyhow::Error;
use biome_service::WorkspaceError;
use std::fmt::{Display, Formatter};
use tower_lsp::lsp_types::MessageType;

#[derive(Debug)]
pub enum LspError {
    WorkspaceError(WorkspaceError),
    Anyhow(anyhow::Error),
}

impl From<WorkspaceError> for LspError {
    fn from(value: WorkspaceError) -> Self {
        Self::WorkspaceError(value)
    }
}

impl From<anyhow::Error> for LspError {
    fn from(value: Error) -> Self {
        Self::Anyhow(value)
    }
}

impl Display for LspError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LspError::WorkspaceError(err) => {
                write!(f, "{}", err)
            }
            LspError::Anyhow(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

/// Receives an error coming from a LSP query, and converts it into a JSON-RPC error.
///
/// It accepts a `Client`, so contextual messages are sent to the user.
pub(crate) async fn handle_lsp_error<T>(
    err: LspError,
    client: &tower_lsp::Client,
) -> Result<Option<T>, tower_lsp::jsonrpc::Error> {
    match err {
        LspError::WorkspaceError(err) => match err {
            // diagnostics that shouldn't raise an hard error, but send a message to the user
            WorkspaceError::FormatWithErrorsDisabled(_)
            | WorkspaceError::FileIgnored(_)
            | WorkspaceError::FileTooLarge(_) => {
                let message = format!("{}", err);
                client.log_message(MessageType::WARNING, message).await;
                Ok(None)
            }

            _ => {
                let message = format!("{}", err);
                client.log_message(MessageType::ERROR, message).await;
                Ok(None)
            }
        },
        LspError::Anyhow(err) => Err(into_lsp_error(err)),
    }
}
