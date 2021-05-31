serve:
  cd server/ && RUST_LOG=pingquery=trace cargo run -- --data=$HOME/foo/data.sql --metadata=$HOME/foo/metadata.sql
