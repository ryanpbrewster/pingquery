import Client, { Config } from "./wrapper";

async function main() {
  const client = new Client("ezdb.fly.dev");
  const CONFIG: Config = {
    queries: [
      {
        name: "get_counts",
        sql_template: "SELECT * FROM word_counts ORDER BY count DESC",
        listen: [{ segments: [] }],
      },
    ],
    mutates: [
      {
        name: "add_word",
        sql_template: `
        INSERT INTO word_counts (word, count) VALUES (:word, 1)
        ON CONFLICT (word) DO UPDATE SET count = count + 1
      `,
        notify: [{ segments: [{ type: "var", name: ":word" }] }],
      },
    ],
  };
  console.log("initializing...");
  await client.init();
  console.log("ensuring tables exist...");
  await client.exec(
    `CREATE TABLE IF NOT EXISTS word_counts (word TEXT PRIMARY KEY, count INTEGER NOT NULL)`
  );
  console.log("setting config...");
  await client.setConfig(CONFIG);

  console.log("setting up stream...");
  const stream = client.interact();
  stream.onData((d) =>
    console.log(`[${performance.now()}] ${JSON.stringify(d)}`)
  );

  stream.send({ id: 1, type: "listen", name: "get_counts" });

  for (var id = 2; ; id++) {
    await new Promise((r) => setTimeout(r, 1_000));
    stream.send({
      id,
      type: "mutate",
      name: "add_word",
      params: { word: "hello" },
    });
  }
}

main().catch((err) => console.error(err));
