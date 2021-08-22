import Client, { Config } from "./wrapper";

async function main() {
  const client = new Client("localhost:8080");
  const CONFIG: Config = {
    queries: [
      {
        name: "get_counts",
        sql_template: "SELECT * FROM word_counts",
        listen: ["all-words"],
      },
    ],
    mutates: [
      {
        name: "add_word",
        sql_template: `
        INSERT INTO word_counts (word, count) VALUES (:word, 1)
        ON CONFLICT (word) DO UPDATE SET count = count + 1
      `,
        notify: ["all-words"],
      },
    ],
  };
  console.log("initializing...");
  await client.init();
  console.log("setting config...");
  await client.setConfig(CONFIG);

  const stream = client.interact();
  stream.onData((d) => console.log(d));
}

main().catch((err) => console.error(err));
