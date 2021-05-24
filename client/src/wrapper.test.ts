import { InteractRequest, InteractResponse } from "./proto/api_pb";
import Client, { Config } from "./wrapper";

const client = new Client("localhost:50051");
const CONFIG: Config = {
  queries: [{ name: "get_counts", sql_template: "SELECT * FROM word_counts" }],
  mutates: [
    {
      name: "set_counts",
      sql_template: "INSERT INTO word_counts (word, count) VALUES (?, 1)",
    },
  ],
};

describe("hello", () => {
  it("should work", () => {
    expect(2 + 2).toEqual(4);
  });
});

describe("config", () => {
  it("read after write", async () => {
    await client.setConfig(CONFIG);
    const fetched = await client.getConfig();
    expect(fetched).toEqual(CONFIG);
  });
});

describe("inspect", () => {
  it("word count", async () => {
    await client.setConfig(CONFIG);

    const stream = client.interact();

    const [promise, resolve] = deferred();

    stream.on("data", (data: InteractResponse) => {
      resolve(data.toObject());
    });

    const w = new InteractRequest();
    stream.write(w);

    console.log("DATA = ", await promise);

    stream.end();
  });
});

function deferred<T>(): [Promise<T>, (v: T) => void] {
  let resolve: (v: T) => void = () => {};
  const promise = new Promise<T>((r) => {
    resolve = r;
  });
  return [promise, resolve];
}
