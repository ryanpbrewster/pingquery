import Client, { Config, InteractResponse } from "./wrapper";

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
    const out = new DeferQueue<InteractResponse>();
    stream.onData((data) => out.push(data));

    await stream.send({ type: "query", id: 1, name: "get_counts" });
    expect(await out.poll()).toEqual({ id: 1, rows: [] });

    await stream.send({
      type: "mutate",
      id: 2,
      name: "add_word",
      params: { word: "hello" },
    });
    expect(await out.poll()).toEqual({ id: 2, rows: [] });

    await stream.send({ type: "query", id: 3, name: "get_counts" });
    expect(await out.poll()).toEqual({
      id: 3,
      rows: [{ word: "hello", count: 1 }],
    });

    stream.end();
  });

  it("word count listen", async () => {
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
    const out = new DeferQueue<InteractResponse>();
    stream.onData((data) => out.push(data));

    await stream.send({ type: "listen", id: 1, name: "get_counts" });
    expect(await out.poll()).toEqual({ id: 1, rows: [] });

    await stream.send({
      type: "mutate",
      id: 2,
      name: "add_word",
      params: { word: "hello" },
    });
    expect(await out.poll()).toEqual({ id: 2, rows: [] });
    expect(await out.poll()).toEqual({
      id: 1,
      rows: [{ word: "hello", count: 1 }],
    });

    stream.end();
  });
});

describe("diagnostics", () => {
  it("smoke test", async () => {
    await client.init();
    const fetched = await client.diagnostics();
    expect(fetched.numConnectedClients).toEqual(0);
  });
});

class Deferred<T> {
  resolve: (value: T) => void = () => {};
  reject: (err: Error) => void = () => {};
  promise: Promise<T>;
  constructor() {
    this.promise = new Promise<T>((res, rej) => {
      this.resolve = res;
      this.reject = rej;
    });
  }
}

class DeferQueue<T> {
  private readonly buf: Deferred<T>[] = [];

  private read: number = 0;
  private write: number = 0;

  private alloc(idx: number): Deferred<T> {
    while (idx >= this.buf.length) {
      this.buf.push(new Deferred());
    }
    return this.buf[idx];
  }
  push(x: T): void {
    this.alloc(this.write++).resolve(x);
  }
  poll(): Promise<T> {
    return this.alloc(this.read++).promise;
  }
}
