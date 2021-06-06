import Wrapper from "../../client/src/wrapper";
import readline from "readline";
async function main() {
  const client = new Wrapper("localhost:50051");
  const stream = client.interact();

  stream.send({ id: 1, type: "listen", name: "get_messages" });
  stream.onData((data) => {
    if (data.id !== 1) return;
    for (const row of data.rows) {
      console.log(`> ${row["content"]}`);
    }
  });

  let id = 2;
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
  });
  rl.on("line", (line) => {
    stream.send({
      id: id++,
      type: "mutate",
      name: "post_message",
      params: { content: line },
    });
  });
}

main().catch((err) => console.log(err));
