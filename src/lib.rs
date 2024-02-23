use anyhow::Context;
pub use oinq::message::client_handshake as handshake;
pub use oinq::request;
use quinn::{RecvStream, SendStream};
use serde::{de::DeserializeOwned, Serialize};

/// Sends a unary request and returns the response.
///
/// # Errors
///
/// Returns an error if there was a problem sending the request or receiving the
/// response.
pub async fn unary_request<I, O>(
    send: &mut SendStream,
    recv: &mut RecvStream,
    code: u32,
    input: I,
) -> anyhow::Result<O>
where
    I: Serialize,
    O: DeserializeOwned,
{
    let mut buf = vec![];
    oinq::message::send_request(send, &mut buf, code, input).await?;

    oinq::frame::recv(recv, &mut buf)
        .await
        .context("invalid response")
}
