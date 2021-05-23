import { ExecRequest, GetConfigRequest } from "./proto/api_pb";
import Client from "./wrapper";

async function main() {
  const client = new Client("localhost:50051");
  await client.getConfig(new GetConfigRequest());

  const execRequest = new ExecRequest();
  execRequest.setRawSql(`SELECT * FROM word_counts`);
  const execResponse = await client.exec(execRequest);
  console.log(execResponse.getRowsList());
}

main().catch((err) => console.error(err));
