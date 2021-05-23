import {
  Config,
  ExecRequest,
  GetConfigRequest,
  SetConfigRequest,
  StatementConfig,
} from "./proto/api_pb";
import Client from "./wrapper";

async function main() {
  const client = new Client("localhost:50051");

  console.log(
    JSON.stringify(
      (await client.getConfig(new GetConfigRequest())).toObject(),
      null,
      2
    )
  );

  const config = new Config();
  const query = new StatementConfig();
  query.setName("get_word_counts");
  query.setSqlTemplate("SELECT * FROM word_counts");
  config.addQueries(query);
  const mutate = new StatementConfig();
  mutate.setName("add_word");
  mutate.setSqlTemplate(`
    INSERT INTO word_counts (word, count) VALUES (?, ?)
    ON CONFLICT (word) DO UPDATE SET count = count + 1
  `);
  config.addMutates(mutate);
  const setConfigRequest = new SetConfigRequest();
  setConfigRequest.setConfig(config);
  console.log(
    JSON.stringify((await client.setConfig(setConfigRequest)).toObject())
  );

  const execResp1 = await client.exec(`
    CREATE TABLE IF NOT EXISTS word_counts (
      word TEXT NOT NULL PRIMARY KEY,
      count INTEGER NOT NULL
    )
  `);
  console.log(JSON.stringify(execResp1.toObject(), null, 2));

  const execResp2 = await client.exec(`SELECT * FROM word_counts`);
  console.log(JSON.stringify(execResp2.toObject(), null, 2));
}

main().catch((err) => console.error(err));
