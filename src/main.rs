use std::env;
use std::cmp;
use std::time::Duration;
use scylla::{Session, SessionBuilder};
use scylla::frame::value::Timestamp;
use futures::stream::StreamExt;

const URI: &str = "172.17.0.2:9042";
const SLEEP_TIME_SEC: u64 = 2;
const SAFE_WINDOW_TIME_SEC: u64 = 1;

const CDC_QUERY: &str = "select pk, t, v, s from ks.t_scylla_cdc_log \
                            where \"cdc$stream_id\" = ? \
                            and \"cdc$time\" >= maxTimeuuid(?) \
                            and \"cdc$time\" < minTimeuuid(?)";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let stream_id_arg = args[1].strip_prefix("0x").expect("Expecting to get stream id as argument");
    let stream_id = hex::decode(stream_id_arg)?;

    let session: Session = SessionBuilder::new()
        .known_node(URI)
        .build()
        .await?;

    // Starting timestamp set to be one hour before now
    let mut from_millis = chrono::Duration::milliseconds(chrono::Local::now().timestamp_millis() - 3_600_000).num_milliseconds();
    let prepared_stmt = session.prepare(CDC_QUERY).await?;

    loop {
        let now_millis = chrono::Local::now().timestamp_millis();
        let safe_time_window_millis: i64 = (SAFE_WINDOW_TIME_SEC * 1000) as i64;
        let to_millis = cmp::max(from_millis, cmp::min(from_millis + safe_time_window_millis, now_millis));

        let mut rows = session.execute_iter(
            prepared_stmt.clone(),
            (&stream_id, Timestamp(chrono::Duration::milliseconds(from_millis)), Timestamp(chrono::Duration::milliseconds(to_millis))),
        )
            .await?
            .into_typed::<(i32, i32, String, String)>();

        while let Some(row) = rows.next().await {
            let (pk_res, t_res, v_res, s_res) = row?;
            println!("pk: {}, t: {}, v: {}, s: {}", pk_res, t_res, v_res, s_res);
        }

        from_millis = to_millis;
        tokio::time::sleep(Duration::new(SLEEP_TIME_SEC, 0));
    }
}
