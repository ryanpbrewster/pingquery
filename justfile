serve:
  cd server/ && RUST_BACKTRACE=1 RUST_LOG=app=trace,pingquery=trace cargo run --bin=app -- --data=$HOME/foo/data.sql --metadata=$HOME/foo/metadata.sql
