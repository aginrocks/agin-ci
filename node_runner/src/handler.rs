use aginci_core::pulsar::ToWorkerMessage;
use color_eyre::eyre::Result;
use pulsar::{Consumer, TokioExecutor, consumer::Message};

pub async fn handle_message(
    consumer: &mut Consumer<ToWorkerMessage, TokioExecutor>,
    msg: Message<ToWorkerMessage>,
) -> Result<()> {
    consumer.ack(&msg).await?;
    let data = msg.deserialize()?;
    dbg!(&data);

    Ok(())
}
