import Client from "./wrapper";

const client = new Client("localhost:50051");

describe("hello", () => {
  it("should work", () => {
    expect(2 + 2).toEqual(4);
  });
});

describe("config", () => {
  it("read after write", async () => {
    const config = {
      queries: [
        { name: "get_counts", sql_template: "SELECT * FROM word_counts" },
      ],
      mutates: [
        {
          name: "set_counts",
          sql_template: "INSERT INTO word_counts (word, count) VALUES (?, 1)",
        },
      ],
    };
    await client.setConfig(config);
    const fetched = await client.getConfig();
    expect(fetched).toEqual(config);
  });
});
