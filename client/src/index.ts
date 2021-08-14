import Client, { Config } from "./wrapper";

async function main() {
  const client = new Client("localhost:50051");
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
  await client.setConfig(CONFIG);
  const stream = client.interact();
}

main().catch((err) => console.error(err));
