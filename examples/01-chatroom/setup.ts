import Wrapper from "../../client/src/wrapper";
async function main() {
  const client = new Wrapper("localhost:50051");
  await client.init();
  await client.exec(`
    CREATE TABLE messages (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        content TEXT NOT NULL
    )
  `);
  await client.setConfig({
    queries: [
      {
        name: "get_messages",
        sql_template: `SELECT * FROM messages`,
        listen: ["all-messages"],
      },
    ],
    mutates: [
      {
        name: "post_message",
        sql_template: "INSERT INTO messages (content) VALUES (:content)",
        notify: ["all-messages"],
      },
    ],
  });
}

main().catch((err) => console.log(err));
