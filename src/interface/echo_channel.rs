use crate::functions::echo_channel;

pub async fn echo_channel_interface() {
    echo_channel::echo_with_channel().await;
}
