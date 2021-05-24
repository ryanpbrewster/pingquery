import { PingQueryClient } from "./proto/api_grpc_pb";
import { credentials } from "@grpc/grpc-js";
import * as api from "./proto/api_pb";

export default class Client {
  private readonly inner: PingQueryClient;
  constructor(address: string) {
    this.inner = new PingQueryClient(address, credentials.createInsecure());
  }

  async getConfig(
    request: api.GetConfigRequest
  ): Promise<api.GetConfigResponse> {
    return new Promise((resolve, reject) =>
      this.inner.getConfig(request, (err, resp) =>
        resp ? resolve(resp) : reject(err)
      )
    );
  }

  async setConfig(
    request: api.SetConfigRequest
  ): Promise<api.SetConfigResponse> {
    return new Promise((resolve, reject) =>
      this.inner.setConfig(request, (err, resp) =>
        resp ? resolve(resp) : reject(err)
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
}

type ObjectMap<T> = { [key: string]: T };
type Value = string | number;
type Row = ObjectMap<Value>;

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
