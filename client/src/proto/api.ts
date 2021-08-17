/* eslint-disable */
import { util, configure, Reader, Writer } from 'protobufjs/minimal';
import * as Long from 'long';
import { Observable } from 'rxjs';

export const protobufPackage = 'pingquery.api';

export interface InitializeRequest {
}

export interface InitializeResponse {
}

export interface DiagnosticsRequest {
}

export interface DiagnosticsResponse {
numConnectedClients: number,
queries: QueryDiagnostics[],
}

export interface QueryDiagnostics {
name: string,
numExecutions: number,
}

export interface GetConfigRequest {
}

export interface GetConfigResponse {
config: Config | undefined,
}

export interface SetConfigRequest {
config: Config | undefined,
}

export interface SetConfigResponse {
}

export interface ExecRequest {
rawSql: string,
}

export interface ExecResponse {
rows: Row[],
}

export interface InteractRequest {
/**
 * / An identifier that the server will echo back with any response related to this request.
 * / Must be monotonically increasing.
 */
id: number,
mutate: Statement | undefined,
query: Statement | undefined,
listen: Statement | undefined,
}

export interface InteractResponse {
/** / The identifier of the request that generated this response. */
id: number,
rows: Row[],
}

export interface Config {
queries: QueryConfig[],
mutates: MutateConfig[],
}

export interface QueryConfig {
name: string,
sqlTemplate: string,
listen: string[],
}

export interface MutateConfig {
name: string,
sqlTemplate: string,
notify: string[],
}

export interface Statement {
name: string,
params: Row | undefined,
}

export interface Value {
integer: number | undefined,
text: string | undefined,
}

export interface Row {
columns: { [key: string ]: Value },
}

export interface Row_ColumnsEntry {
key: string,
value: Value | undefined,
}

const baseInitializeRequest: object = {  };

export const InitializeRequest = {
            encode(
      _: InitializeRequest,
      writer: Writer = Writer.create(),
    ): Writer {
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): InitializeRequest {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseInitializeRequest } as InitializeRequest;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(_: any): InitializeRequest {
      const message = { ...baseInitializeRequest } as InitializeRequest;
return message
},

toJSON(_: InitializeRequest): unknown {
      const obj: any = {};
return obj;
},

fromPartial(_: DeepPartial<InitializeRequest>): InitializeRequest {
      const message = { ...baseInitializeRequest } as InitializeRequest;
return message;
}
          };

const baseInitializeResponse: object = {  };

export const InitializeResponse = {
            encode(
      _: InitializeResponse,
      writer: Writer = Writer.create(),
    ): Writer {
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): InitializeResponse {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseInitializeResponse } as InitializeResponse;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(_: any): InitializeResponse {
      const message = { ...baseInitializeResponse } as InitializeResponse;
return message
},

toJSON(_: InitializeResponse): unknown {
      const obj: any = {};
return obj;
},

fromPartial(_: DeepPartial<InitializeResponse>): InitializeResponse {
      const message = { ...baseInitializeResponse } as InitializeResponse;
return message;
}
          };

const baseDiagnosticsRequest: object = {  };

export const DiagnosticsRequest = {
            encode(
      _: DiagnosticsRequest,
      writer: Writer = Writer.create(),
    ): Writer {
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): DiagnosticsRequest {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseDiagnosticsRequest } as DiagnosticsRequest;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(_: any): DiagnosticsRequest {
      const message = { ...baseDiagnosticsRequest } as DiagnosticsRequest;
return message
},

toJSON(_: DiagnosticsRequest): unknown {
      const obj: any = {};
return obj;
},

fromPartial(_: DeepPartial<DiagnosticsRequest>): DiagnosticsRequest {
      const message = { ...baseDiagnosticsRequest } as DiagnosticsRequest;
return message;
}
          };

const baseDiagnosticsResponse: object = { numConnectedClients: 0 };

export const DiagnosticsResponse = {
            encode(
      message: DiagnosticsResponse,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.numConnectedClients !== 0) {
          writer.uint32(8).int32(message.numConnectedClients);
        }
for (const v of message.queries) {
            QueryDiagnostics.encode(v!, writer.uint32(18).fork()).ldelim();
          }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): DiagnosticsResponse {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseDiagnosticsResponse } as DiagnosticsResponse;
message.queries = [];
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.numConnectedClients = reader.int32();
break;
case 2:
message.queries.push(QueryDiagnostics.decode(reader, reader.uint32()));
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): DiagnosticsResponse {
      const message = { ...baseDiagnosticsResponse } as DiagnosticsResponse;
message.queries = [];
if (object.numConnectedClients !== undefined && object.numConnectedClients !== null) {
message.numConnectedClients = Number(object.numConnectedClients);
} else {
message.numConnectedClients = 0;
}
if (object.queries !== undefined && object.queries !== null) {
for (const e of object.queries) {
            message.queries.push(QueryDiagnostics.fromJSON(e));
          }
}
return message
},

toJSON(message: DiagnosticsResponse): unknown {
      const obj: any = {};
message.numConnectedClients !== undefined && (obj.numConnectedClients = message.numConnectedClients);
if (message.queries) {
          obj.queries = message.queries.map(e => e ? QueryDiagnostics.toJSON(e) : undefined);
        } else {
          obj.queries = [];
        }
return obj;
},

fromPartial(object: DeepPartial<DiagnosticsResponse>): DiagnosticsResponse {
      const message = { ...baseDiagnosticsResponse } as DiagnosticsResponse;
message.queries = [];
if (object.numConnectedClients !== undefined && object.numConnectedClients !== null) {
message.numConnectedClients = object.numConnectedClients;
} else {
message.numConnectedClients = 0
}
if (object.queries !== undefined && object.queries !== null) {
for (const e of object.queries) {
            message.queries.push(QueryDiagnostics.fromPartial(e));
          }
}
return message;
}
          };

const baseQueryDiagnostics: object = { name: "",numExecutions: 0 };

export const QueryDiagnostics = {
            encode(
      message: QueryDiagnostics,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.name !== "") {
          writer.uint32(10).string(message.name);
        }
if (message.numExecutions !== 0) {
          writer.uint32(16).int64(message.numExecutions);
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): QueryDiagnostics {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseQueryDiagnostics } as QueryDiagnostics;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.name = reader.string();
break;
case 2:
message.numExecutions = longToNumber(reader.int64() as Long);
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): QueryDiagnostics {
      const message = { ...baseQueryDiagnostics } as QueryDiagnostics;
if (object.name !== undefined && object.name !== null) {
message.name = String(object.name);
} else {
message.name = "";
}
if (object.numExecutions !== undefined && object.numExecutions !== null) {
message.numExecutions = Number(object.numExecutions);
} else {
message.numExecutions = 0;
}
return message
},

toJSON(message: QueryDiagnostics): unknown {
      const obj: any = {};
message.name !== undefined && (obj.name = message.name);
message.numExecutions !== undefined && (obj.numExecutions = message.numExecutions);
return obj;
},

fromPartial(object: DeepPartial<QueryDiagnostics>): QueryDiagnostics {
      const message = { ...baseQueryDiagnostics } as QueryDiagnostics;
if (object.name !== undefined && object.name !== null) {
message.name = object.name;
} else {
message.name = ""
}
if (object.numExecutions !== undefined && object.numExecutions !== null) {
message.numExecutions = object.numExecutions;
} else {
message.numExecutions = 0
}
return message;
}
          };

const baseGetConfigRequest: object = {  };

export const GetConfigRequest = {
            encode(
      _: GetConfigRequest,
      writer: Writer = Writer.create(),
    ): Writer {
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): GetConfigRequest {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseGetConfigRequest } as GetConfigRequest;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(_: any): GetConfigRequest {
      const message = { ...baseGetConfigRequest } as GetConfigRequest;
return message
},

toJSON(_: GetConfigRequest): unknown {
      const obj: any = {};
return obj;
},

fromPartial(_: DeepPartial<GetConfigRequest>): GetConfigRequest {
      const message = { ...baseGetConfigRequest } as GetConfigRequest;
return message;
}
          };

const baseGetConfigResponse: object = {  };

export const GetConfigResponse = {
            encode(
      message: GetConfigResponse,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.config !== undefined) {
          Config.encode(message.config, writer.uint32(10).fork()).ldelim();
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): GetConfigResponse {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseGetConfigResponse } as GetConfigResponse;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.config = Config.decode(reader, reader.uint32());
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): GetConfigResponse {
      const message = { ...baseGetConfigResponse } as GetConfigResponse;
if (object.config !== undefined && object.config !== null) {
message.config = Config.fromJSON(object.config);
} else {
message.config = undefined;
}
return message
},

toJSON(message: GetConfigResponse): unknown {
      const obj: any = {};
message.config !== undefined && (obj.config = message.config ? Config.toJSON(message.config) : undefined);
return obj;
},

fromPartial(object: DeepPartial<GetConfigResponse>): GetConfigResponse {
      const message = { ...baseGetConfigResponse } as GetConfigResponse;
if (object.config !== undefined && object.config !== null) {
message.config = Config.fromPartial(object.config);
} else {
message.config = undefined
}
return message;
}
          };

const baseSetConfigRequest: object = {  };

export const SetConfigRequest = {
            encode(
      message: SetConfigRequest,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.config !== undefined) {
          Config.encode(message.config, writer.uint32(10).fork()).ldelim();
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): SetConfigRequest {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseSetConfigRequest } as SetConfigRequest;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.config = Config.decode(reader, reader.uint32());
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): SetConfigRequest {
      const message = { ...baseSetConfigRequest } as SetConfigRequest;
if (object.config !== undefined && object.config !== null) {
message.config = Config.fromJSON(object.config);
} else {
message.config = undefined;
}
return message
},

toJSON(message: SetConfigRequest): unknown {
      const obj: any = {};
message.config !== undefined && (obj.config = message.config ? Config.toJSON(message.config) : undefined);
return obj;
},

fromPartial(object: DeepPartial<SetConfigRequest>): SetConfigRequest {
      const message = { ...baseSetConfigRequest } as SetConfigRequest;
if (object.config !== undefined && object.config !== null) {
message.config = Config.fromPartial(object.config);
} else {
message.config = undefined
}
return message;
}
          };

const baseSetConfigResponse: object = {  };

export const SetConfigResponse = {
            encode(
      _: SetConfigResponse,
      writer: Writer = Writer.create(),
    ): Writer {
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): SetConfigResponse {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseSetConfigResponse } as SetConfigResponse;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(_: any): SetConfigResponse {
      const message = { ...baseSetConfigResponse } as SetConfigResponse;
return message
},

toJSON(_: SetConfigResponse): unknown {
      const obj: any = {};
return obj;
},

fromPartial(_: DeepPartial<SetConfigResponse>): SetConfigResponse {
      const message = { ...baseSetConfigResponse } as SetConfigResponse;
return message;
}
          };

const baseExecRequest: object = { rawSql: "" };

export const ExecRequest = {
            encode(
      message: ExecRequest,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.rawSql !== "") {
          writer.uint32(10).string(message.rawSql);
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): ExecRequest {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseExecRequest } as ExecRequest;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.rawSql = reader.string();
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): ExecRequest {
      const message = { ...baseExecRequest } as ExecRequest;
if (object.rawSql !== undefined && object.rawSql !== null) {
message.rawSql = String(object.rawSql);
} else {
message.rawSql = "";
}
return message
},

toJSON(message: ExecRequest): unknown {
      const obj: any = {};
message.rawSql !== undefined && (obj.rawSql = message.rawSql);
return obj;
},

fromPartial(object: DeepPartial<ExecRequest>): ExecRequest {
      const message = { ...baseExecRequest } as ExecRequest;
if (object.rawSql !== undefined && object.rawSql !== null) {
message.rawSql = object.rawSql;
} else {
message.rawSql = ""
}
return message;
}
          };

const baseExecResponse: object = {  };

export const ExecResponse = {
            encode(
      message: ExecResponse,
      writer: Writer = Writer.create(),
    ): Writer {
for (const v of message.rows) {
            Row.encode(v!, writer.uint32(10).fork()).ldelim();
          }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): ExecResponse {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseExecResponse } as ExecResponse;
message.rows = [];
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.rows.push(Row.decode(reader, reader.uint32()));
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): ExecResponse {
      const message = { ...baseExecResponse } as ExecResponse;
message.rows = [];
if (object.rows !== undefined && object.rows !== null) {
for (const e of object.rows) {
            message.rows.push(Row.fromJSON(e));
          }
}
return message
},

toJSON(message: ExecResponse): unknown {
      const obj: any = {};
if (message.rows) {
          obj.rows = message.rows.map(e => e ? Row.toJSON(e) : undefined);
        } else {
          obj.rows = [];
        }
return obj;
},

fromPartial(object: DeepPartial<ExecResponse>): ExecResponse {
      const message = { ...baseExecResponse } as ExecResponse;
message.rows = [];
if (object.rows !== undefined && object.rows !== null) {
for (const e of object.rows) {
            message.rows.push(Row.fromPartial(e));
          }
}
return message;
}
          };

const baseInteractRequest: object = { id: 0 };

export const InteractRequest = {
            encode(
      message: InteractRequest,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.id !== 0) {
          writer.uint32(8).int32(message.id);
        }
if (message.mutate !== undefined) {
          Statement.encode(message.mutate, writer.uint32(18).fork()).ldelim();
        }
if (message.query !== undefined) {
          Statement.encode(message.query, writer.uint32(26).fork()).ldelim();
        }
if (message.listen !== undefined) {
          Statement.encode(message.listen, writer.uint32(34).fork()).ldelim();
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): InteractRequest {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseInteractRequest } as InteractRequest;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.id = reader.int32();
break;
case 2:
message.mutate = Statement.decode(reader, reader.uint32());
break;
case 3:
message.query = Statement.decode(reader, reader.uint32());
break;
case 4:
message.listen = Statement.decode(reader, reader.uint32());
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): InteractRequest {
      const message = { ...baseInteractRequest } as InteractRequest;
if (object.id !== undefined && object.id !== null) {
message.id = Number(object.id);
} else {
message.id = 0;
}
if (object.mutate !== undefined && object.mutate !== null) {
message.mutate = Statement.fromJSON(object.mutate);
} else {
message.mutate = undefined;
}
if (object.query !== undefined && object.query !== null) {
message.query = Statement.fromJSON(object.query);
} else {
message.query = undefined;
}
if (object.listen !== undefined && object.listen !== null) {
message.listen = Statement.fromJSON(object.listen);
} else {
message.listen = undefined;
}
return message
},

toJSON(message: InteractRequest): unknown {
      const obj: any = {};
message.id !== undefined && (obj.id = message.id);
message.mutate !== undefined && (obj.mutate = message.mutate ? Statement.toJSON(message.mutate) : undefined);
message.query !== undefined && (obj.query = message.query ? Statement.toJSON(message.query) : undefined);
message.listen !== undefined && (obj.listen = message.listen ? Statement.toJSON(message.listen) : undefined);
return obj;
},

fromPartial(object: DeepPartial<InteractRequest>): InteractRequest {
      const message = { ...baseInteractRequest } as InteractRequest;
if (object.id !== undefined && object.id !== null) {
message.id = object.id;
} else {
message.id = 0
}
if (object.mutate !== undefined && object.mutate !== null) {
message.mutate = Statement.fromPartial(object.mutate);
} else {
message.mutate = undefined
}
if (object.query !== undefined && object.query !== null) {
message.query = Statement.fromPartial(object.query);
} else {
message.query = undefined
}
if (object.listen !== undefined && object.listen !== null) {
message.listen = Statement.fromPartial(object.listen);
} else {
message.listen = undefined
}
return message;
}
          };

const baseInteractResponse: object = { id: 0 };

export const InteractResponse = {
            encode(
      message: InteractResponse,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.id !== 0) {
          writer.uint32(8).int32(message.id);
        }
for (const v of message.rows) {
            Row.encode(v!, writer.uint32(18).fork()).ldelim();
          }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): InteractResponse {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseInteractResponse } as InteractResponse;
message.rows = [];
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.id = reader.int32();
break;
case 2:
message.rows.push(Row.decode(reader, reader.uint32()));
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): InteractResponse {
      const message = { ...baseInteractResponse } as InteractResponse;
message.rows = [];
if (object.id !== undefined && object.id !== null) {
message.id = Number(object.id);
} else {
message.id = 0;
}
if (object.rows !== undefined && object.rows !== null) {
for (const e of object.rows) {
            message.rows.push(Row.fromJSON(e));
          }
}
return message
},

toJSON(message: InteractResponse): unknown {
      const obj: any = {};
message.id !== undefined && (obj.id = message.id);
if (message.rows) {
          obj.rows = message.rows.map(e => e ? Row.toJSON(e) : undefined);
        } else {
          obj.rows = [];
        }
return obj;
},

fromPartial(object: DeepPartial<InteractResponse>): InteractResponse {
      const message = { ...baseInteractResponse } as InteractResponse;
message.rows = [];
if (object.id !== undefined && object.id !== null) {
message.id = object.id;
} else {
message.id = 0
}
if (object.rows !== undefined && object.rows !== null) {
for (const e of object.rows) {
            message.rows.push(Row.fromPartial(e));
          }
}
return message;
}
          };

const baseConfig: object = {  };

export const Config = {
            encode(
      message: Config,
      writer: Writer = Writer.create(),
    ): Writer {
for (const v of message.queries) {
            QueryConfig.encode(v!, writer.uint32(10).fork()).ldelim();
          }
for (const v of message.mutates) {
            MutateConfig.encode(v!, writer.uint32(18).fork()).ldelim();
          }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): Config {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseConfig } as Config;
message.queries = [];
message.mutates = [];
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.queries.push(QueryConfig.decode(reader, reader.uint32()));
break;
case 2:
message.mutates.push(MutateConfig.decode(reader, reader.uint32()));
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

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
return message
},

toJSON(message: Config): unknown {
      const obj: any = {};
if (message.queries) {
          obj.queries = message.queries.map(e => e ? QueryConfig.toJSON(e) : undefined);
        } else {
          obj.queries = [];
        }
if (message.mutates) {
          obj.mutates = message.mutates.map(e => e ? MutateConfig.toJSON(e) : undefined);
        } else {
          obj.mutates = [];
        }
return obj;
},

fromPartial(object: DeepPartial<Config>): Config {
      const message = { ...baseConfig } as Config;
message.queries = [];
message.mutates = [];
if (object.queries !== undefined && object.queries !== null) {
for (const e of object.queries) {
            message.queries.push(QueryConfig.fromPartial(e));
          }
}
if (object.mutates !== undefined && object.mutates !== null) {
for (const e of object.mutates) {
            message.mutates.push(MutateConfig.fromPartial(e));
          }
}
return message;
}
          };

const baseQueryConfig: object = { name: "",sqlTemplate: "",listen: "" };

export const QueryConfig = {
            encode(
      message: QueryConfig,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.name !== "") {
          writer.uint32(10).string(message.name);
        }
if (message.sqlTemplate !== "") {
          writer.uint32(18).string(message.sqlTemplate);
        }
for (const v of message.listen) {
            writer.uint32(26).string(v!);
          }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): QueryConfig {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseQueryConfig } as QueryConfig;
message.listen = [];
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.name = reader.string();
break;
case 2:
message.sqlTemplate = reader.string();
break;
case 3:
message.listen.push(reader.string());
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): QueryConfig {
      const message = { ...baseQueryConfig } as QueryConfig;
message.listen = [];
if (object.name !== undefined && object.name !== null) {
message.name = String(object.name);
} else {
message.name = "";
}
if (object.sqlTemplate !== undefined && object.sqlTemplate !== null) {
message.sqlTemplate = String(object.sqlTemplate);
} else {
message.sqlTemplate = "";
}
if (object.listen !== undefined && object.listen !== null) {
for (const e of object.listen) {
            message.listen.push(String(e));
          }
}
return message
},

toJSON(message: QueryConfig): unknown {
      const obj: any = {};
message.name !== undefined && (obj.name = message.name);
message.sqlTemplate !== undefined && (obj.sqlTemplate = message.sqlTemplate);
if (message.listen) {
          obj.listen = message.listen.map(e => e);
        } else {
          obj.listen = [];
        }
return obj;
},

fromPartial(object: DeepPartial<QueryConfig>): QueryConfig {
      const message = { ...baseQueryConfig } as QueryConfig;
message.listen = [];
if (object.name !== undefined && object.name !== null) {
message.name = object.name;
} else {
message.name = ""
}
if (object.sqlTemplate !== undefined && object.sqlTemplate !== null) {
message.sqlTemplate = object.sqlTemplate;
} else {
message.sqlTemplate = ""
}
if (object.listen !== undefined && object.listen !== null) {
for (const e of object.listen) {
            message.listen.push(e);
          }
}
return message;
}
          };

const baseMutateConfig: object = { name: "",sqlTemplate: "",notify: "" };

export const MutateConfig = {
            encode(
      message: MutateConfig,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.name !== "") {
          writer.uint32(10).string(message.name);
        }
if (message.sqlTemplate !== "") {
          writer.uint32(18).string(message.sqlTemplate);
        }
for (const v of message.notify) {
            writer.uint32(26).string(v!);
          }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): MutateConfig {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseMutateConfig } as MutateConfig;
message.notify = [];
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.name = reader.string();
break;
case 2:
message.sqlTemplate = reader.string();
break;
case 3:
message.notify.push(reader.string());
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): MutateConfig {
      const message = { ...baseMutateConfig } as MutateConfig;
message.notify = [];
if (object.name !== undefined && object.name !== null) {
message.name = String(object.name);
} else {
message.name = "";
}
if (object.sqlTemplate !== undefined && object.sqlTemplate !== null) {
message.sqlTemplate = String(object.sqlTemplate);
} else {
message.sqlTemplate = "";
}
if (object.notify !== undefined && object.notify !== null) {
for (const e of object.notify) {
            message.notify.push(String(e));
          }
}
return message
},

toJSON(message: MutateConfig): unknown {
      const obj: any = {};
message.name !== undefined && (obj.name = message.name);
message.sqlTemplate !== undefined && (obj.sqlTemplate = message.sqlTemplate);
if (message.notify) {
          obj.notify = message.notify.map(e => e);
        } else {
          obj.notify = [];
        }
return obj;
},

fromPartial(object: DeepPartial<MutateConfig>): MutateConfig {
      const message = { ...baseMutateConfig } as MutateConfig;
message.notify = [];
if (object.name !== undefined && object.name !== null) {
message.name = object.name;
} else {
message.name = ""
}
if (object.sqlTemplate !== undefined && object.sqlTemplate !== null) {
message.sqlTemplate = object.sqlTemplate;
} else {
message.sqlTemplate = ""
}
if (object.notify !== undefined && object.notify !== null) {
for (const e of object.notify) {
            message.notify.push(e);
          }
}
return message;
}
          };

const baseStatement: object = { name: "" };

export const Statement = {
            encode(
      message: Statement,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.name !== "") {
          writer.uint32(10).string(message.name);
        }
if (message.params !== undefined) {
          Row.encode(message.params, writer.uint32(18).fork()).ldelim();
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): Statement {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseStatement } as Statement;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.name = reader.string();
break;
case 2:
message.params = Row.decode(reader, reader.uint32());
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): Statement {
      const message = { ...baseStatement } as Statement;
if (object.name !== undefined && object.name !== null) {
message.name = String(object.name);
} else {
message.name = "";
}
if (object.params !== undefined && object.params !== null) {
message.params = Row.fromJSON(object.params);
} else {
message.params = undefined;
}
return message
},

toJSON(message: Statement): unknown {
      const obj: any = {};
message.name !== undefined && (obj.name = message.name);
message.params !== undefined && (obj.params = message.params ? Row.toJSON(message.params) : undefined);
return obj;
},

fromPartial(object: DeepPartial<Statement>): Statement {
      const message = { ...baseStatement } as Statement;
if (object.name !== undefined && object.name !== null) {
message.name = object.name;
} else {
message.name = ""
}
if (object.params !== undefined && object.params !== null) {
message.params = Row.fromPartial(object.params);
} else {
message.params = undefined
}
return message;
}
          };

const baseValue: object = {  };

export const Value = {
            encode(
      message: Value,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.integer !== undefined) {
          writer.uint32(8).int64(message.integer);
        }
if (message.text !== undefined) {
          writer.uint32(18).string(message.text);
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): Value {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseValue } as Value;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.integer = longToNumber(reader.int64() as Long);
break;
case 2:
message.text = reader.string();
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): Value {
      const message = { ...baseValue } as Value;
if (object.integer !== undefined && object.integer !== null) {
message.integer = Number(object.integer);
} else {
message.integer = undefined;
}
if (object.text !== undefined && object.text !== null) {
message.text = String(object.text);
} else {
message.text = undefined;
}
return message
},

toJSON(message: Value): unknown {
      const obj: any = {};
message.integer !== undefined && (obj.integer = message.integer);
message.text !== undefined && (obj.text = message.text);
return obj;
},

fromPartial(object: DeepPartial<Value>): Value {
      const message = { ...baseValue } as Value;
if (object.integer !== undefined && object.integer !== null) {
message.integer = object.integer;
} else {
message.integer = undefined
}
if (object.text !== undefined && object.text !== null) {
message.text = object.text;
} else {
message.text = undefined
}
return message;
}
          };

const baseRow: object = {  };

export const Row = {
            encode(
      message: Row,
      writer: Writer = Writer.create(),
    ): Writer {
Object.entries(message.columns).forEach(([key, value]) => {
            Row_ColumnsEntry.encode({  key: key as any, value }, writer.uint32(10).fork()).ldelim();
          });
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): Row {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseRow } as Row;
message.columns = {};
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
const entry1 = Row_ColumnsEntry.decode(reader, reader.uint32());
          if (entry1.value !== undefined) {
            message.columns[entry1.key] = entry1.value;
          }
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): Row {
      const message = { ...baseRow } as Row;
message.columns = {};
if (object.columns !== undefined && object.columns !== null) {
Object.entries(object.columns).forEach(([key, value]) => {
            message.columns[key] = Value.fromJSON(value);
          });
}
return message
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

fromPartial(object: DeepPartial<Row>): Row {
      const message = { ...baseRow } as Row;
message.columns = {};
if (object.columns !== undefined && object.columns !== null) {
Object.entries(object.columns).forEach(([key, value]) => {
            if (value !== undefined) {
              message.columns[key] = Value.fromPartial(value);
            }
          });
}
return message;
}
          };

const baseRow_ColumnsEntry: object = { key: "" };

export const Row_ColumnsEntry = {
            encode(
      message: Row_ColumnsEntry,
      writer: Writer = Writer.create(),
    ): Writer {
if (message.key !== "") {
          writer.uint32(10).string(message.key);
        }
if (message.value !== undefined) {
          Value.encode(message.value, writer.uint32(18).fork()).ldelim();
        }
return writer;
},

decode(
      input: Reader | Uint8Array,
      length?: number,
    ): Row_ColumnsEntry {
      const reader = input instanceof Reader ? input : new Reader(input);
      let end = length === undefined ? reader.len : reader.pos + length;
      const message = { ...baseRow_ColumnsEntry } as Row_ColumnsEntry;
while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
case 1:
message.key = reader.string();
break;
case 2:
message.value = Value.decode(reader, reader.uint32());
break;
default:
      reader.skipType(tag & 7);
      break;
}
}
return message;
},

fromJSON(object: any): Row_ColumnsEntry {
      const message = { ...baseRow_ColumnsEntry } as Row_ColumnsEntry;
if (object.key !== undefined && object.key !== null) {
message.key = String(object.key);
} else {
message.key = "";
}
if (object.value !== undefined && object.value !== null) {
message.value = Value.fromJSON(object.value);
} else {
message.value = undefined;
}
return message
},

toJSON(message: Row_ColumnsEntry): unknown {
      const obj: any = {};
message.key !== undefined && (obj.key = message.key);
message.value !== undefined && (obj.value = message.value ? Value.toJSON(message.value) : undefined);
return obj;
},

fromPartial(object: DeepPartial<Row_ColumnsEntry>): Row_ColumnsEntry {
      const message = { ...baseRow_ColumnsEntry } as Row_ColumnsEntry;
if (object.key !== undefined && object.key !== null) {
message.key = object.key;
} else {
message.key = ""
}
if (object.value !== undefined && object.value !== null) {
message.value = Value.fromPartial(object.value);
} else {
message.value = undefined
}
return message;
}
          };

export interface PingQuery {
Initialize(request: InitializeRequest): Promise<InitializeResponse>;
Diagnostics(request: DiagnosticsRequest): Promise<DiagnosticsResponse>;
GetConfig(request: GetConfigRequest): Promise<GetConfigResponse>;
SetConfig(request: SetConfigRequest): Promise<SetConfigResponse>;
Exec(request: ExecRequest): Promise<ExecResponse>;
Interact(request: Observable<InteractRequest>): Observable<InteractResponse>;
}

export class PingQueryClientImpl implements PingQuery {private readonly rpc: Rpc;constructor(rpc: Rpc) {this.rpc = rpc;this.Initialize = this.Initialize.bind(this);this.Diagnostics = this.Diagnostics.bind(this);this.GetConfig = this.GetConfig.bind(this);this.SetConfig = this.SetConfig.bind(this);this.Exec = this.Exec.bind(this);this.Interact = this.Interact.bind(this);}
    Initialize(
      request: InitializeRequest
    ): Promise<InitializeResponse> {
      const data = InitializeRequest.encode(request).finish();
      const promise = this.rpc.request(
        
        "pingquery.api.PingQuery",
        "Initialize",
        data
      );
      return promise.then(data => InitializeResponse.decode(new Reader(data)));
    }
  
    Diagnostics(
      request: DiagnosticsRequest
    ): Promise<DiagnosticsResponse> {
      const data = DiagnosticsRequest.encode(request).finish();
      const promise = this.rpc.request(
        
        "pingquery.api.PingQuery",
        "Diagnostics",
        data
      );
      return promise.then(data => DiagnosticsResponse.decode(new Reader(data)));
    }
  
    GetConfig(
      request: GetConfigRequest
    ): Promise<GetConfigResponse> {
      const data = GetConfigRequest.encode(request).finish();
      const promise = this.rpc.request(
        
        "pingquery.api.PingQuery",
        "GetConfig",
        data
      );
      return promise.then(data => GetConfigResponse.decode(new Reader(data)));
    }
  
    SetConfig(
      request: SetConfigRequest
    ): Promise<SetConfigResponse> {
      const data = SetConfigRequest.encode(request).finish();
      const promise = this.rpc.request(
        
        "pingquery.api.PingQuery",
        "SetConfig",
        data
      );
      return promise.then(data => SetConfigResponse.decode(new Reader(data)));
    }
  
    Exec(
      request: ExecRequest
    ): Promise<ExecResponse> {
      const data = ExecRequest.encode(request).finish();
      const promise = this.rpc.request(
        
        "pingquery.api.PingQuery",
        "Exec",
        data
      );
      return promise.then(data => ExecResponse.decode(new Reader(data)));
    }
  
    Interact(
      request: Observable<InteractRequest>
    ): Promise<InteractResponse> {
      const data = Observable<InteractRequest>.encode(request).finish();
      const promise = this.rpc.request(
        
        "pingquery.api.PingQuery",
        "Interact",
        data
      );
      return promise.then(data => InteractResponse.decode(new Reader(data)));
    }
  }

interface Rpc {
      request(
        
        service: string,
        method: string,
        data: Uint8Array
      ): Promise<Uint8Array>;
    }

declare var self: any | undefined;
      declare var window: any | undefined;
      var globalThis: any = (() => {
        if (typeof globalThis !== "undefined") return globalThis;
        if (typeof self !== "undefined") return self;
        if (typeof window !== "undefined") return window;
        if (typeof global !== "undefined") return global;
        throw "Unable to locate global object";
      })();





type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;
      export type DeepPartial<T> = T extends Builtin
        ? T
        : T extends Array<infer U>
        ? Array<DeepPartial<U>>
        : T extends ReadonlyArray<infer U>
        ? ReadonlyArray<DeepPartial<U>>
        : T extends {}
        ? { [K in keyof T]?: DeepPartial<T[K]> }
        : Partial<T>;









function longToNumber(long: Long): number {
        if (long.gt(Number.MAX_SAFE_INTEGER)) {
          throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER")
        }
        return long.toNumber();
      }



// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
    // add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
      if (util.Long !== Long) {
        util.Long = Long as any;
        configure();
      }

