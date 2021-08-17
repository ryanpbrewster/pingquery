import * as api from "./proto/api";
import got from "got";

export default class Client {
  constructor(private readonly address: string) {}
  private async doGet<T>(path: string): Promise<T> {
    return got.get(`http://${this.address}/${path}`).json();
  }
  private async doPost<T>(path: string, json: object): Promise<T> {
    const resp = got.post(`http://${this.address}/${path}`, { json });
    return await resp.json();
  }

  async init(): Promise<void> {
    const req: api.InitializeRequest = {};
    await this.doPost("initialize", req);
  }
  async diagnostics(): Promise<Diagnostics> {
    return await this.doGet("diagnostics");
  }
  async getConfig(): Promise<Config | null> {
    return configFromProto(await this.doGet("config"));
  }
  async setConfig(config: Config): Promise<void> {
    const request: api.SetConfigRequest = { config: configToProto(config) };
    return await this.doPost("config", request);
  }
  async exec(raw_sql: string): Promise<Row[]> {
    const request: api.ExecRequest = { rawSql: raw_sql };
    return await this.doPost("exec", request);
  }
  interact(): InteractWrapper {
    return new InteractWrapper();
  }
}

class InteractWrapper {
  constructor() {}

  send(req: InteractRequest): Promise<void> {
    const proto = interactRequestToProto(req);
    return new Promise((resolve) => {
      // this.inner.write(proto, () => resolve());
    });
  }
  end() {
    // this.inner.end();
  }
  onData(cb: (resp: InteractResponse) => void): void {
    // this.inner.on("data", (chunk) => cb(interactResponseFromProto(chunk)));
  }
  onEnd(cb: () => void): void {
    // this.inner.on("end", () => cb());
  }
  onError(cb: (resp: Error) => void): void {
    // this.inner.on("error", (err) => cb(err));
  }
}

export interface Config {
  readonly queries: readonly QueryConfig[];
  readonly mutates: readonly MutateConfig[];
}
export interface QueryConfig {
  readonly name: string;
  readonly sql_template: string;
  readonly listen?: readonly string[];
}
export interface MutateConfig {
  readonly name: string;
  readonly sql_template: string;
  readonly notify?: readonly string[];
}

export interface Diagnostics {
  readonly numConnectedClients: number;
  readonly queries: ReadonlyMap<string, QueryDiagnostics>;
}
export interface QueryDiagnostics {
  readonly numExecutions: number;
}

export type InteractRequest = Query | Mutate | Listen;
export interface Query {
  readonly type: "query";
  readonly id: number;
  readonly name: string;
  readonly params?: Row;
}
export interface Mutate {
  readonly type: "mutate";
  readonly id: number;
  readonly name: string;
  readonly params?: Row;
}
export interface Listen {
  readonly type: "listen";
  readonly id: number;
  readonly name: string;
  readonly params?: Row;
}

export interface InteractResponse {
  readonly id: number;
  readonly rows: Row[];
}

export type ObjectMap<T> = { [key: string]: T };
export type Value = string | number;
export type Row = ObjectMap<Value>;

function interactRequestToProto(req: InteractRequest): api.InteractRequest {
  const proto: api.InteractRequest = {
    id: req.id,
    query: undefined,
    mutate: undefined,
    listen: undefined,
  };
  switch (req.type) {
    case "query":
      proto.query = queryToProto(req);
      return proto;
    case "mutate":
      proto.mutate = mutateToProto(req);
      return proto;
    case "listen":
      proto.listen = listenToProto(req);
      return proto;
  }
}
function queryToProto(query: Query): api.Statement {
  return {
    name: query.name,
    params: query.params ? paramsToProto(query.params) : undefined,
  };
}
function mutateToProto(mutate: Mutate): api.Statement {
  return {
    name: mutate.name,
    params: mutate.params ? paramsToProto(mutate.params) : undefined,
  };
}
function listenToProto(listen: Listen): api.Statement {
  return {
    name: listen.name,
    params: listen.params ? paramsToProto(listen.params) : undefined,
  };
}
function interactResponseFromProto(
  proto: api.InteractResponse
): InteractResponse {
  return {
    id: proto.id,
    rows: proto.rows.map(rowFromProto),
  };
}
function valueToProto(v: Value): api.Value {
  switch (typeof v) {
    case "number":
      return { integer: v, text: undefined };
    case "string":
      return { text: v, integer: undefined };
  }
}
function valueFromProto(p: api.Value): Value | null {
  if (p.integer !== undefined) return p.integer;
  if (p.text !== undefined) return p.text;
  return null;
}

function rowFromProto(p: api.Row): Row {
  const out: Row = {};
  Object.entries(p.columns).forEach(([k, v]) => {
    const value = valueFromProto(v);
    if (value) {
      out[k] = value;
    }
  });
  return out;
}
function paramsToProto(params: Row): api.Row {
  const proto: api.Row = { columns: {} };
  for (const [k, v] of Object.entries(params)) {
    proto.columns[`:${k}`] = valueToProto(v);
  }
  return proto;
}

function configToProto(config: Config): api.Config {
  return {
    queries: config.queries.map((q) => queryConfigToProto(q)),
    mutates: config.mutates.map((m) => mutateConfigToProto(m)),
  };
}

function queryConfigToProto(q: QueryConfig): api.QueryConfig {
  return {
    name: q.name,
    sqlTemplate: q.sql_template,
    listen: q.listen ? [...q.listen] : [],
  };
}

function mutateConfigToProto(m: MutateConfig): api.MutateConfig {
  return {
    name: m.name,
    sqlTemplate: m.sql_template,
    notify: m.notify ? [...m.notify] : [],
  };
}

function configFromProto(p: api.GetConfigResponse): Config | null {
  return {
    queries: p.config?.queries.map(queryConfigFromProto) || [],
    mutates: p.config?.mutates.map(mutateConfigFromProto) || [],
  };
}

function queryConfigFromProto(p: api.QueryConfig): QueryConfig {
  return {
    name: p.name,
    sql_template: p.sqlTemplate,
    listen: p.listen,
  };
}

function mutateConfigFromProto(p: api.MutateConfig): MutateConfig {
  return {
    name: p.name,
    sql_template: p.sqlTemplate,
    notify: p.notify,
  };
}

function diagnosticsFromProto(p: api.DiagnosticsResponse): Diagnostics | null {
  return {
    numConnectedClients: p.numConnectedClients,
    queries: new Map(
      p.queries.map((p) => [p.name, queryDiagnosticsFromProto(p)])
    ),
  };
}
function queryDiagnosticsFromProto(p: api.QueryDiagnostics): QueryDiagnostics {
  return {
    numExecutions: p.numExecutions,
  };
}
