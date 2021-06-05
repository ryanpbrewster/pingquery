import Client, { Config, InteractResponse } from "./wrapper";

const client = new Client("localhost:50051");
const CONFIG: Config = {
  queries: [{ name: "get_counts", sql_template: "SELECT * FROM word_counts" }],
  mutates: [
    {
      name: "add_word",
      sql_template: `
        INSERT INTO word_counts (word, count) VALUES (:word, 1)
        ON CONFLICT (word) DO UPDATE SET count = count + 1
      `,
    },
  ],
};

describe("config", () => {
  it("read after write", async () => {
    await client.init();
    await client.setConfig(CONFIG);
    const fetched = await client.getConfig();
    expect(fetched).toEqual(CONFIG);
  });
});

describe("inspect", () => {
  it("word count", async () => {
    await client.init();
    await client.setConfig(CONFIG);
    await client.exec(`DROP TABLE IF EXISTS word_counts`);
    await client.exec(`
      CREATE TABLE IF NOT EXISTS word_counts (
        word TEXT PRIMARY KEY,
        count INTEGER NOT NULL
      )
    `);

    const stream = client.interact();

    const d0 = deferred<InteractResponse>();
    stream.onData((data) => d0.resolve(data));
    await stream.send({ type: "query", id: 1, name: "get_counts" });
    expect(await d0.promise).toEqual({ id: 1, rows: [] });

    const d1 = deferred<InteractResponse>();
    stream.onData((data) => d1.resolve(data));
    await stream.send({
      type: "mutate",
      id: 2,
      name: "add_word",
      params: { ":word": "hello" },
    });
    expect(await d1.promise).toEqual({ id: 2, rows: [] });

    const d2 = deferred<InteractResponse>();
    stream.onData((data) => d2.resolve(data));
    await stream.send({ type: "query", id: 3, name: "get_counts" });
    expect(await d2.promise).toEqual({
      id: 3,
      rows: [{ word: "hello", count: 1 }],
    });

    stream.end();
  });
});

interface Deferred<T> {
  readonly promise: Promise<T>;
  resolve(value: T): void;
  reject(err: Error): void;
}
function deferred<T>(): Deferred<T> {
  let resolve: (value: T) => void = () => {};
  let reject: (err: Error) => void = () => {};
  let promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
}
