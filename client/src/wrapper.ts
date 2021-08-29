import * as api from "./proto/api";
import got from "got";
import WebSocket from "ws";

export default class Client {
  constructor(private readonly address: string) {}
  private async doGet<T>(path: string): Promise<T> {
    // console.log(`[GET] ${path}`);
    return got.get(`http://${this.address}/${path}`).json();
  }
  private async doPost<T>(path: string, json: object): Promise<T> {
    // console.log(`[POST] ${path} -> ${JSON.stringify(json)}`);
    const resp = got.post(`http://${this.address}/${path}`, { json });
    return await resp.json();
  }

  async init(): Promise<void> {
    const req: api.InitializeRequest = {};
    await this.doPost(
      "initialize",
      api.InitializeRequest.toJSON(req) as object
    );
  }
  async diagnostics(): Promise<Diagnostics> {
    const resp: api.DiagnosticsResponse = await this.doGet("diagnostics");
    return diagnosticsFromProto(resp)!;
  }
  async getConfig(): Promise<Config | null> {
    return configFromProto(await this.doGet("config"));
  }
  async setConfig(config: Config): Promise<void> {
    const request: api.SetConfigRequest = { config: configToProto(config) };
    return await this.doPost(
      "config",
      api.SetConfigRequest.toJSON(request) as object
    );
  }
  async exec(raw_sql: string): Promise<Row[]> {
    const request: api.ExecRequest = { rawSql: raw_sql };
    return await this.doPost("exec", api.ExecRequest.toJSON(request) as object);
  }
  interact(): InteractWrapper {
    return new InteractWrapper(this.address);
  }
}

class InteractWrapper {
  private readonly socket: Deferred<WebSocket> = new Deferred();
  private dataCb: (resp: InteractResponse) => void = () => {};
  private endCb: () => void = () => {};
  private errorCb: (err: Error) => void = () => {};

  constructor(address: string) {
    const socket = new WebSocket(`ws://${address}/interact`);
    socket.on("open", () => {
      this.socket.resolve(socket);
    });
    socket.on("message", (data) => {
      const msg = (data as Buffer).toString();
      // console.log("[RECV]", msg);
      this.dataCb(
        interactResponseFromProto(
          api.InteractResponse.fromJSON(JSON.parse(msg))
        )
      );
    });
    socket.on("error", (err) => {
      // console.error(err);
      this.errorCb(err);
    });
    socket.on("close", (reason) => {
      this.endCb();
    });
  }

  async send(req: InteractRequest): Promise<void> {
    const msg = JSON.stringify(
      api.InteractRequest.toJSON(interactRequestToProto(req))
    );
    const socket = await this.socket.promise;
    // console.log(`[SEND] ${msg}`);
    socket.send(msg);
  }
  async end(): Promise<void> {
    const socket = await this.socket.promise;
    socket.close();
  }
  onData(cb: (resp: InteractResponse) => void): void {
    this.dataCb = cb;
  }
  onEnd(cb: () => void): void {
    this.endCb = cb;
  }
  onError(cb: (resp: Error) => void): void {
    this.errorCb = cb;
  }
}

export interface Config {
  readonly queries: readonly QueryConfig[];
  readonly mutates: readonly MutateConfig[];
}
export interface QueryConfig {
  readonly name: string;
  readonly sql_template: string;
  readonly listen?: readonly Path[];
}
export interface MutateConfig {
  readonly name: string;
  readonly sql_template: string;
  readonly notify?: readonly Path[];
}
export interface Path {
  readonly segments: readonly Segment[];
}
namespace path {
  export interface Lit {
    readonly type: "lit";
    readonly value: string;
  }
  export interface Var {
    readonly type: "var";
    readonly name: string;
  }
}
export type Segment = path.Lit | path.Var;

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
      return { integer: v, text: "" };
    case "string":
      return { text: v, integer: 0 };
  }
}
function valueFromProto(p: api.Value): Value | null {
  if (p.integer) return p.integer;
  if (p.text) return p.text;
  return null;
}

function rowFromProto(p: api.Row): Row {
  const out: Row = {};
  Object.entries(p.columns).forEach(([k, v]) => {
    const value = valueFromProto(v);
    if (value !== null) {
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
    listen: q.listen ? q.listen.map((p) => pathToProto(p)) : [],
  };
}

function mutateConfigToProto(m: MutateConfig): api.MutateConfig {
  return {
    name: m.name,
    sqlTemplate: m.sql_template,
    notify: m.notify ? m.notify.map((p) => pathToProto(p)) : [],
  };
}

function pathToProto(path: Path): api.Path {
  return {
    segments: path.segments.map((segment) => segmentToProto(segment)),
  };
}
function segmentToProto(segment: Segment): string {
  switch (segment.type) {
    case "lit":
      return segment.value;
    case "var":
      return `{${segment.name}}`;
  }
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
    listen: p.listen.map((path) => pathFromProto(path)),
  };
}

function mutateConfigFromProto(p: api.MutateConfig): MutateConfig {
  return {
    name: p.name,
    sql_template: p.sqlTemplate,
    notify: p.notify.map((path) => pathFromProto(path)),
  };
}
function pathFromProto(proto: api.Path): Path {
  return {
    segments: proto.segments.map((segment) => segmentFromProto(segment)),
  };
}

function segmentFromProto(segment: string): Segment {
  if (segment.startsWith("{") && segment.endsWith("}")) {
    return { type: "var", name: segment.substring(1, segment.length - 1) };
  } else {
    return { type: "lit", value: segment };
  }
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

export class Deferred<T> {
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
