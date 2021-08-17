/* eslint-disable */
import Long from "long";
import _m0 from "protobufjs/minimal";

export const protobufPackage = "pingquery.api";

export interface InitializeRequest {}

export interface InitializeResponse {}

export interface DiagnosticsRequest {}

export interface DiagnosticsResponse {
  numConnectedClients: number;
  queries: QueryDiagnostics[];
}

export interface QueryDiagnostics {
  name: string;
  numExecutions: number;
}

export interface GetConfigRequest {}

export interface GetConfigResponse {
  config: Config | undefined;
}

export interface SetConfigRequest {
  config: Config | undefined;
}

export interface SetConfigResponse {}

export interface ExecRequest {
  rawSql: string;
}

export interface ExecResponse {
  rows: Row[];
}

export interface InteractRequest {
  /**
   * / An identifier that the server will echo back with any response related to this request.
   * / Must be monotonically increasing.
   */
  id: number;
  mutate: Statement | undefined;
  query: Statement | undefined;
  listen: Statement | undefined;
}

export interface InteractResponse {
  /** / The identifier of the request that generated this response. */
  id: number;
  rows: Row[];
}

export interface Config {
  queries: QueryConfig[];
  mutates: MutateConfig[];
}

export interface QueryConfig {
  name: string;
  sqlTemplate: string;
  listen: string[];
}

export interface MutateConfig {
  name: string;
  sqlTemplate: string;
  notify: string[];
}

export interface Statement {
  name: string;
  params: Row | undefined;
}

export interface Value {
  integer: number;
  text: string;
}

export interface Row {
  columns: { [key: string]: Value };
}

export interface Row_ColumnsEntry {
  key: string;
  value: Value | undefined;
}

const baseInitializeRequest: object = {};

export const InitializeRequest = {
  fromJSON(_: any): InitializeRequest {
    const message = { ...baseInitializeRequest } as InitializeRequest;
    return message;
  },

  toJSON(_: InitializeRequest): unknown {
    const obj: any = {};
    return obj;
  },
};

const baseInitializeResponse: object = {};

export const InitializeResponse = {
  fromJSON(_: any): InitializeResponse {
    const message = { ...baseInitializeResponse } as InitializeResponse;
    return message;
  },

  toJSON(_: InitializeResponse): unknown {
    const obj: any = {};
    return obj;
  },
};

const baseDiagnosticsRequest: object = {};

export const DiagnosticsRequest = {
  fromJSON(_: any): DiagnosticsRequest {
    const message = { ...baseDiagnosticsRequest } as DiagnosticsRequest;
    return message;
  },

  toJSON(_: DiagnosticsRequest): unknown {
    const obj: any = {};
    return obj;
  },
};

const baseDiagnosticsResponse: object = { numConnectedClients: 0 };

export const DiagnosticsResponse = {
  fromJSON(object: any): DiagnosticsResponse {
    const message = { ...baseDiagnosticsResponse } as DiagnosticsResponse;
    message.queries = [];
    if (
      object.numConnectedClients !== undefined &&
      object.numConnectedClients !== null
    ) {
      message.numConnectedClients = Number(object.numConnectedClients);
    }
    if (object.queries !== undefined && object.queries !== null) {
      for (const e of object.queries) {
        message.queries.push(QueryDiagnostics.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: DiagnosticsResponse): unknown {
    const obj: any = {};
    message.numConnectedClients !== undefined &&
      (obj.numConnectedClients = message.numConnectedClients);
    if (message.queries) {
      obj.queries = message.queries.map((e) =>
        e ? QueryDiagnostics.toJSON(e) : undefined
      );
    } else {
      obj.queries = [];
    }
    return obj;
  },
};

const baseQueryDiagnostics: object = { name: "", numExecutions: 0 };

export const QueryDiagnostics = {
  fromJSON(object: any): QueryDiagnostics {
    const message = { ...baseQueryDiagnostics } as QueryDiagnostics;
    if (object.name !== undefined && object.name !== null) {
      message.name = String(object.name);
    }
    if (object.numExecutions !== undefined && object.numExecutions !== null) {
      message.numExecutions = Number(object.numExecutions);
    }
    return message;
  },

  toJSON(message: QueryDiagnostics): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.numExecutions !== undefined &&
      (obj.numExecutions = message.numExecutions);
    return obj;
  },
};

const baseGetConfigRequest: object = {};

export const GetConfigRequest = {
  fromJSON(_: any): GetConfigRequest {
    const message = { ...baseGetConfigRequest } as GetConfigRequest;
    return message;
  },

  toJSON(_: GetConfigRequest): unknown {
    const obj: any = {};
    return obj;
  },
};

const baseGetConfigResponse: object = {};

export const GetConfigResponse = {
  fromJSON(object: any): GetConfigResponse {
    const message = { ...baseGetConfigResponse } as GetConfigResponse;
    if (object.config !== undefined && object.config !== null) {
      message.config = Config.fromJSON(object.config);
    }
    return message;
  },

  toJSON(message: GetConfigResponse): unknown {
    const obj: any = {};
    message.config !== undefined &&
      (obj.config = message.config ? Config.toJSON(message.config) : undefined);
    return obj;
  },
};

const baseSetConfigRequest: object = {};

export const SetConfigRequest = {
  fromJSON(object: any): SetConfigRequest {
    const message = { ...baseSetConfigRequest } as SetConfigRequest;
    if (object.config !== undefined && object.config !== null) {
      message.config = Config.fromJSON(object.config);
    }
    return message;
  },

  toJSON(message: SetConfigRequest): unknown {
    const obj: any = {};
    message.config !== undefined &&
      (obj.config = message.config ? Config.toJSON(message.config) : undefined);
    return obj;
  },
};

const baseSetConfigResponse: object = {};

export const SetConfigResponse = {
  fromJSON(_: any): SetConfigResponse {
    const message = { ...baseSetConfigResponse } as SetConfigResponse;
    return message;
  },

  toJSON(_: SetConfigResponse): unknown {
    const obj: any = {};
    return obj;
  },
};

const baseExecRequest: object = { rawSql: "" };

export const ExecRequest = {
  fromJSON(object: any): ExecRequest {
    const message = { ...baseExecRequest } as ExecRequest;
    if (object.rawSql !== undefined && object.rawSql !== null) {
      message.rawSql = String(object.rawSql);
    }
    return message;
  },

  toJSON(message: ExecRequest): unknown {
    const obj: any = {};
    message.rawSql !== undefined && (obj.rawSql = message.rawSql);
    return obj;
  },
};

const baseExecResponse: object = {};

export const ExecResponse = {
  fromJSON(object: any): ExecResponse {
    const message = { ...baseExecResponse } as ExecResponse;
    message.rows = [];
    if (object.rows !== undefined && object.rows !== null) {
      for (const e of object.rows) {
        message.rows.push(Row.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: ExecResponse): unknown {
    const obj: any = {};
    if (message.rows) {
      obj.rows = message.rows.map((e) => (e ? Row.toJSON(e) : undefined));
    } else {
      obj.rows = [];
    }
    return obj;
  },
};

const baseInteractRequest: object = { id: 0 };

export const InteractRequest = {
  fromJSON(object: any): InteractRequest {
    const message = { ...baseInteractRequest } as InteractRequest;
    if (object.id !== undefined && object.id !== null) {
      message.id = Number(object.id);
    }
    if (object.mutate !== undefined && object.mutate !== null) {
      message.mutate = Statement.fromJSON(object.mutate);
    }
    if (object.query !== undefined && object.query !== null) {
      message.query = Statement.fromJSON(object.query);
    }
    if (object.listen !== undefined && object.listen !== null) {
      message.listen = Statement.fromJSON(object.listen);
    }
    return message;
  },

  toJSON(message: InteractRequest): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.mutate !== undefined &&
      (obj.mutate = message.mutate
        ? Statement.toJSON(message.mutate)
        : undefined);
    message.query !== undefined &&
      (obj.query = message.query ? Statement.toJSON(message.query) : undefined);
    message.listen !== undefined &&
      (obj.listen = message.listen
        ? Statement.toJSON(message.listen)
        : undefined);
    return obj;
  },
};

const baseInteractResponse: object = { id: 0 };

export const InteractResponse = {
  fromJSON(object: any): InteractResponse {
    const message = { ...baseInteractResponse } as InteractResponse;
    message.rows = [];
    if (object.id !== undefined && object.id !== null) {
      message.id = Number(object.id);
    }
    if (object.rows !== undefined && object.rows !== null) {
      for (const e of object.rows) {
        message.rows.push(Row.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: InteractResponse): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    if (message.rows) {
      obj.rows = message.rows.map((e) => (e ? Row.toJSON(e) : undefined));
    } else {
      obj.rows = [];
    }
    return obj;
  },
};

const baseConfig: object = {};

export const Config = {
  fromJSON(object: any): Config {
    const message = { ...baseConfig } as Config;
    message.queries = [];
    message.mutates = [];
    if (object.queries !== undefined && object.queries !== null) {
      for (const e of object.queries) {
        message.queries.push(QueryConfig.fromJSON(e));
      }
    }
    if (object.mutates !== undefined && object.mutates !== null) {
      for (const e of object.mutates) {
        message.mutates.push(MutateConfig.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: Config): unknown {
    const obj: any = {};
    if (message.queries) {
      obj.queries = message.queries.map((e) =>
        e ? QueryConfig.toJSON(e) : undefined
      );
    } else {
      obj.queries = [];
    }
    if (message.mutates) {
      obj.mutates = message.mutates.map((e) =>
        e ? MutateConfig.toJSON(e) : undefined
      );
    } else {
      obj.mutates = [];
    }
    return obj;
  },
};

const baseQueryConfig: object = { name: "", sqlTemplate: "", listen: "" };

export const QueryConfig = {
  fromJSON(object: any): QueryConfig {
    const message = { ...baseQueryConfig } as QueryConfig;
    message.listen = [];
    if (object.name !== undefined && object.name !== null) {
      message.name = String(object.name);
    }
    if (object.sqlTemplate !== undefined && object.sqlTemplate !== null) {
      message.sqlTemplate = String(object.sqlTemplate);
    }
    if (object.listen !== undefined && object.listen !== null) {
      for (const e of object.listen) {
        message.listen.push(String(e));
      }
    }
    return message;
  },

  toJSON(message: QueryConfig): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.sqlTemplate !== undefined &&
      (obj.sqlTemplate = message.sqlTemplate);
    if (message.listen) {
      obj.listen = message.listen.map((e) => e);
    } else {
      obj.listen = [];
    }
    return obj;
  },
};

const baseMutateConfig: object = { name: "", sqlTemplate: "", notify: "" };

export const MutateConfig = {
  fromJSON(object: any): MutateConfig {
    const message = { ...baseMutateConfig } as MutateConfig;
    message.notify = [];
    if (object.name !== undefined && object.name !== null) {
      message.name = String(object.name);
    }
    if (object.sqlTemplate !== undefined && object.sqlTemplate !== null) {
      message.sqlTemplate = String(object.sqlTemplate);
    }
    if (object.notify !== undefined && object.notify !== null) {
      for (const e of object.notify) {
        message.notify.push(String(e));
      }
    }
    return message;
  },

  toJSON(message: MutateConfig): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.sqlTemplate !== undefined &&
      (obj.sqlTemplate = message.sqlTemplate);
    if (message.notify) {
      obj.notify = message.notify.map((e) => e);
    } else {
      obj.notify = [];
    }
    return obj;
  },
};

const baseStatement: object = { name: "" };

export const Statement = {
  fromJSON(object: any): Statement {
    const message = { ...baseStatement } as Statement;
    if (object.name !== undefined && object.name !== null) {
      message.name = String(object.name);
    }
    if (object.params !== undefined && object.params !== null) {
      message.params = Row.fromJSON(object.params);
    }
    return message;
  },

  toJSON(message: Statement): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.params !== undefined &&
      (obj.params = message.params ? Row.toJSON(message.params) : undefined);
    return obj;
  },
};

const baseValue: object = { integer: 0, text: "" };

export const Value = {
  fromJSON(object: any): Value {
    const message = { ...baseValue } as Value;
    if (object.integer !== undefined && object.integer !== null) {
      message.integer = Number(object.integer);
    }
    if (object.text !== undefined && object.text !== null) {
      message.text = String(object.text);
    }
    return message;
  },

  toJSON(message: Value): unknown {
    const obj: any = {};
    message.integer !== undefined && (obj.integer = message.integer);
    message.text !== undefined && (obj.text = message.text);
    return obj;
  },
};

const baseRow: object = {};

export const Row = {
  fromJSON(object: any): Row {
    const message = { ...baseRow } as Row;
    message.columns = {};
    if (object.columns !== undefined && object.columns !== null) {
      Object.entries(object.columns).forEach(([key, value]) => {
        message.columns[key] = Value.fromJSON(value);
      });
    }
    return message;
  },

  toJSON(message: Row): unknown {
    const obj: any = {};
    obj.columns = {};
    if (message.columns) {
      Object.entries(message.columns).forEach(([k, v]) => {
        obj.columns[k] = Value.toJSON(v);
      });
    }
    return obj;
  },
};

const baseRow_ColumnsEntry: object = { key: "" };

export const Row_ColumnsEntry = {
  fromJSON(object: any): Row_ColumnsEntry {
    const message = { ...baseRow_ColumnsEntry } as Row_ColumnsEntry;
    if (object.key !== undefined && object.key !== null) {
      message.key = String(object.key);
    }
    if (object.value !== undefined && object.value !== null) {
      message.value = Value.fromJSON(object.value);
    }
    return message;
  },

  toJSON(message: Row_ColumnsEntry): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value ? Value.toJSON(message.value) : undefined);
    return obj;
  },
};

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}
