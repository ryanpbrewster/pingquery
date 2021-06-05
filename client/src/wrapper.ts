import { PingQueryClient } from "./proto/api_grpc_pb";
import {
  ClientDuplexStream,
  ClientWritableStream,
  credentials,
} from "@grpc/grpc-js";
import * as api from "./proto/api_pb";

export default class Client {
  private readonly inner: PingQueryClient;
  constructor(address: string) {
    this.inner = new PingQueryClient(address, credentials.createInsecure());
  }

  async init(): Promise<void> {
    return new Promise((resolve, reject) =>
      this.inner.initialize(new api.InitializeRequest(), (err, resp) =>
        resp ? resolve() : reject(err)
      )
    );
  }
  async getConfig(): Promise<Config> {
    return new Promise((resolve, reject) =>
      this.inner.getConfig(new api.GetConfigRequest(), (err, resp) =>
        resp ? resolve(configFromProto(resp.getConfig()!)!) : reject(err)
      )
    );
  }

  async setConfig(config: Config): Promise<void> {
    const request = new api.SetConfigRequest();
    request.setConfig(configToProto(config));
    return new Promise((resolve, reject) =>
      this.inner.setConfig(request, (err, resp) =>
        resp ? resolve() : reject(err)
      )
    );
  }

  async exec(raw_sql: string): Promise<Row[]> {
    const req = new api.ExecRequest();
    req.setRawSql(raw_sql);
    return new Promise((resolve, reject) =>
      this.inner.exec(req, (err, resp) => {
        return resp
          ? resolve(resp.getRowsList().map(rowFromProto))
          : reject(err);
      })
    );
  }

  interact(): InteractWrapper {
    return new InteractWrapper(this.inner.interact());
  }
}

class InteractWrapper {
  constructor(
    private readonly inner: ClientDuplexStream<
      api.InteractRequest,
      api.InteractResponse
    >
  ) {}

  send(req: InteractRequest): Promise<void> {
    const proto = interactRequestToProto(req);
    return new Promise((resolve) => {
      this.inner.write(proto, () => resolve());
    });
  }
  end() {
    this.inner.end();
  }
  onData(cb: (resp: InteractResponse) => void): void {
    this.inner.on("data", (chunk) => cb(interactResponseFromProto(chunk)));
  }
  onEnd(cb: () => void): void {
    this.inner.on("end", () => cb());
  }
  onError(cb: (resp: Error) => void): void {
    this.inner.on("error", (err) => cb(err));
  }
}

export interface Config {
  readonly queries: readonly QueryConfig[];
  readonly mutates: readonly MutateConfig[];
}
export interface QueryConfig {
  readonly name: string;
  readonly sql_template: string;
}
export interface MutateConfig {
  readonly name: string;
  readonly sql_template: string;
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
  const proto = new api.InteractRequest();
  proto.setId(req.id);
  switch (req.type) {
    case "query":
      proto.setQuery(queryToProto(req));
      return proto;
    case "mutate":
      proto.setMutate(mutateToProto(req));
      return proto;
    case "listen":
      proto.setListen(listenToProto(req));
      return proto;
  }
}
function queryToProto(query: Query): api.Statement {
  const proto = new api.Statement();
  proto.setName(query.name);
  if (query.params) {
    proto.setParams(rowToProto(query.params));
  }
  return proto;
}
function mutateToProto(mutate: Mutate): api.Statement {
  const proto = new api.Statement();
  proto.setName(mutate.name);
  if (mutate.params) {
    proto.setParams(rowToProto(mutate.params));
  }
  return proto;
}
function listenToProto(listen: Listen): api.Statement {
  const proto = new api.Statement();
  proto.setName(listen.name);
  if (listen.params) {
    proto.setParams(rowToProto(listen.params));
  }
  return proto;
}
function interactResponseFromProto(
  proto: api.InteractResponse
): InteractResponse {
  return {
    id: proto.getId(),
    rows: proto.getRowsList().map(rowFromProto),
  };
}
function valueToProto(v: Value): api.Value {
  const p = new api.Value();
  switch (typeof v) {
    case "number":
      p.setInteger(v);
      break;
    case "string":
      p.setText(v);
      break;
  }
  return p;
}
function valueFromProto(p: api.Value): Value | null {
  switch (p.getTypeCase()) {
    case api.Value.TypeCase.TYPE_NOT_SET:
      return null;
    case api.Value.TypeCase.INTEGER:
      return p.getInteger();
    case api.Value.TypeCase.TEXT:
      return p.getText();
  }
}

function rowFromProto(p: api.Row): Row {
  const out: Row = {};
  p.getColumnsMap().forEach((v, k) => {
    const value = valueFromProto(v);
    if (value) {
      out[k] = value;
    }
  });
  return out;
}
function rowToProto(params: Row): api.Row {
  const proto = new api.Row();
  for (const [k, v] of Object.entries(params)) {
    proto.getColumnsMap().set(k, valueToProto(v));
  }
  return proto;
}

function configToProto(config: Config): api.Config {
  const p = new api.Config();
  for (const q of config.queries) {
    p.addQueries(queryConfigToProto(q));
  }
  for (const m of config.mutates) {
    p.addMutates(mutateConfigToProto(m));
  }
  return p;
}

function queryConfigToProto(q: QueryConfig): api.StatementConfig {
  const p = new api.StatementConfig();
  p.setName(q.name);
  p.setSqlTemplate(q.sql_template);
  return p;
}

function mutateConfigToProto(m: MutateConfig): api.StatementConfig {
  const p = new api.StatementConfig();
  p.setName(m.name);
  p.setSqlTemplate(m.sql_template);
  return p;
}

function configFromProto(p: api.Config): Config | null {
  return {
    queries: p.getQueriesList().map(queryConfigFromProto),
    mutates: p.getMutatesList().map(mutateConfigFromProto),
  };
}

function queryConfigFromProto(p: api.StatementConfig): QueryConfig {
  return {
    name: p.getName(),
    sql_template: p.getSqlTemplate(),
  };
}

function mutateConfigFromProto(p: api.StatementConfig): MutateConfig {
  return {
    name: p.getName(),
    sql_template: p.getSqlTemplate(),
  };
}
