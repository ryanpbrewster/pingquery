// package: pingquery.api
// file: api.proto

import * as jspb from "google-protobuf";

export class InitializeRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InitializeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: InitializeRequest): InitializeRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InitializeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InitializeRequest;
  static deserializeBinaryFromReader(message: InitializeRequest, reader: jspb.BinaryReader): InitializeRequest;
}

export namespace InitializeRequest {
  export type AsObject = {
  }
}

export class InitializeResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InitializeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: InitializeResponse): InitializeResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InitializeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InitializeResponse;
  static deserializeBinaryFromReader(message: InitializeResponse, reader: jspb.BinaryReader): InitializeResponse;
}

export namespace InitializeResponse {
  export type AsObject = {
  }
}

export class DiagnosticsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DiagnosticsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DiagnosticsRequest): DiagnosticsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DiagnosticsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DiagnosticsRequest;
  static deserializeBinaryFromReader(message: DiagnosticsRequest, reader: jspb.BinaryReader): DiagnosticsRequest;
}

export namespace DiagnosticsRequest {
  export type AsObject = {
  }
}

export class DiagnosticsResponse extends jspb.Message {
  getNumConnectedClients(): number;
  setNumConnectedClients(value: number): void;

  clearQueriesList(): void;
  getQueriesList(): Array<QueryDiagnostics>;
  setQueriesList(value: Array<QueryDiagnostics>): void;
  addQueries(value?: QueryDiagnostics, index?: number): QueryDiagnostics;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DiagnosticsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DiagnosticsResponse): DiagnosticsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DiagnosticsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DiagnosticsResponse;
  static deserializeBinaryFromReader(message: DiagnosticsResponse, reader: jspb.BinaryReader): DiagnosticsResponse;
}

export namespace DiagnosticsResponse {
  export type AsObject = {
    numConnectedClients: number,
    queriesList: Array<QueryDiagnostics.AsObject>,
  }
}

export class QueryDiagnostics extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getNumExecutions(): number;
  setNumExecutions(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): QueryDiagnostics.AsObject;
  static toObject(includeInstance: boolean, msg: QueryDiagnostics): QueryDiagnostics.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: QueryDiagnostics, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): QueryDiagnostics;
  static deserializeBinaryFromReader(message: QueryDiagnostics, reader: jspb.BinaryReader): QueryDiagnostics;
}

export namespace QueryDiagnostics {
  export type AsObject = {
    name: string,
    numExecutions: number,
  }
}

export class GetConfigRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetConfigRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetConfigRequest): GetConfigRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetConfigRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetConfigRequest;
  static deserializeBinaryFromReader(message: GetConfigRequest, reader: jspb.BinaryReader): GetConfigRequest;
}

export namespace GetConfigRequest {
  export type AsObject = {
  }
}

export class GetConfigResponse extends jspb.Message {
  hasConfig(): boolean;
  clearConfig(): void;
  getConfig(): Config | undefined;
  setConfig(value?: Config): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetConfigResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetConfigResponse): GetConfigResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetConfigResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetConfigResponse;
  static deserializeBinaryFromReader(message: GetConfigResponse, reader: jspb.BinaryReader): GetConfigResponse;
}

export namespace GetConfigResponse {
  export type AsObject = {
    config?: Config.AsObject,
  }
}

export class SetConfigRequest extends jspb.Message {
  hasConfig(): boolean;
  clearConfig(): void;
  getConfig(): Config | undefined;
  setConfig(value?: Config): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetConfigRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetConfigRequest): SetConfigRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetConfigRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetConfigRequest;
  static deserializeBinaryFromReader(message: SetConfigRequest, reader: jspb.BinaryReader): SetConfigRequest;
}

export namespace SetConfigRequest {
  export type AsObject = {
    config?: Config.AsObject,
  }
}

export class SetConfigResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetConfigResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SetConfigResponse): SetConfigResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetConfigResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetConfigResponse;
  static deserializeBinaryFromReader(message: SetConfigResponse, reader: jspb.BinaryReader): SetConfigResponse;
}

export namespace SetConfigResponse {
  export type AsObject = {
  }
}

export class ExecRequest extends jspb.Message {
  getRawSql(): string;
  setRawSql(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExecRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ExecRequest): ExecRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ExecRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExecRequest;
  static deserializeBinaryFromReader(message: ExecRequest, reader: jspb.BinaryReader): ExecRequest;
}

export namespace ExecRequest {
  export type AsObject = {
    rawSql: string,
  }
}

export class ExecResponse extends jspb.Message {
  clearRowsList(): void;
  getRowsList(): Array<Row>;
  setRowsList(value: Array<Row>): void;
  addRows(value?: Row, index?: number): Row;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExecResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ExecResponse): ExecResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ExecResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExecResponse;
  static deserializeBinaryFromReader(message: ExecResponse, reader: jspb.BinaryReader): ExecResponse;
}

export namespace ExecResponse {
  export type AsObject = {
    rowsList: Array<Row.AsObject>,
  }
}

export class InteractRequest extends jspb.Message {
  getId(): number;
  setId(value: number): void;

  hasMutate(): boolean;
  clearMutate(): void;
  getMutate(): Statement | undefined;
  setMutate(value?: Statement): void;

  hasQuery(): boolean;
  clearQuery(): void;
  getQuery(): Statement | undefined;
  setQuery(value?: Statement): void;

  hasListen(): boolean;
  clearListen(): void;
  getListen(): Statement | undefined;
  setListen(value?: Statement): void;

  getTypeCase(): InteractRequest.TypeCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InteractRequest.AsObject;
  static toObject(includeInstance: boolean, msg: InteractRequest): InteractRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InteractRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InteractRequest;
  static deserializeBinaryFromReader(message: InteractRequest, reader: jspb.BinaryReader): InteractRequest;
}

export namespace InteractRequest {
  export type AsObject = {
    id: number,
    mutate?: Statement.AsObject,
    query?: Statement.AsObject,
    listen?: Statement.AsObject,
  }

  export enum TypeCase {
    TYPE_NOT_SET = 0,
    MUTATE = 2,
    QUERY = 3,
    LISTEN = 4,
  }
}

export class InteractResponse extends jspb.Message {
  getId(): number;
  setId(value: number): void;

  clearRowsList(): void;
  getRowsList(): Array<Row>;
  setRowsList(value: Array<Row>): void;
  addRows(value?: Row, index?: number): Row;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InteractResponse.AsObject;
  static toObject(includeInstance: boolean, msg: InteractResponse): InteractResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InteractResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InteractResponse;
  static deserializeBinaryFromReader(message: InteractResponse, reader: jspb.BinaryReader): InteractResponse;
}

export namespace InteractResponse {
  export type AsObject = {
    id: number,
    rowsList: Array<Row.AsObject>,
  }
}

export class Config extends jspb.Message {
  clearQueriesList(): void;
  getQueriesList(): Array<QueryConfig>;
  setQueriesList(value: Array<QueryConfig>): void;
  addQueries(value?: QueryConfig, index?: number): QueryConfig;

  clearMutatesList(): void;
  getMutatesList(): Array<MutateConfig>;
  setMutatesList(value: Array<MutateConfig>): void;
  addMutates(value?: MutateConfig, index?: number): MutateConfig;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Config.AsObject;
  static toObject(includeInstance: boolean, msg: Config): Config.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Config, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Config;
  static deserializeBinaryFromReader(message: Config, reader: jspb.BinaryReader): Config;
}

export namespace Config {
  export type AsObject = {
    queriesList: Array<QueryConfig.AsObject>,
    mutatesList: Array<MutateConfig.AsObject>,
  }
}

export class QueryConfig extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getSqlTemplate(): string;
  setSqlTemplate(value: string): void;

  clearListenList(): void;
  getListenList(): Array<string>;
  setListenList(value: Array<string>): void;
  addListen(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): QueryConfig.AsObject;
  static toObject(includeInstance: boolean, msg: QueryConfig): QueryConfig.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: QueryConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): QueryConfig;
  static deserializeBinaryFromReader(message: QueryConfig, reader: jspb.BinaryReader): QueryConfig;
}

export namespace QueryConfig {
  export type AsObject = {
    name: string,
    sqlTemplate: string,
    listenList: Array<string>,
  }
}

export class MutateConfig extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getSqlTemplate(): string;
  setSqlTemplate(value: string): void;

  clearNotifyList(): void;
  getNotifyList(): Array<string>;
  setNotifyList(value: Array<string>): void;
  addNotify(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MutateConfig.AsObject;
  static toObject(includeInstance: boolean, msg: MutateConfig): MutateConfig.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: MutateConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MutateConfig;
  static deserializeBinaryFromReader(message: MutateConfig, reader: jspb.BinaryReader): MutateConfig;
}

export namespace MutateConfig {
  export type AsObject = {
    name: string,
    sqlTemplate: string,
    notifyList: Array<string>,
  }
}

export class Statement extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  hasParams(): boolean;
  clearParams(): void;
  getParams(): Row | undefined;
  setParams(value?: Row): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Statement.AsObject;
  static toObject(includeInstance: boolean, msg: Statement): Statement.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Statement, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Statement;
  static deserializeBinaryFromReader(message: Statement, reader: jspb.BinaryReader): Statement;
}

export namespace Statement {
  export type AsObject = {
    name: string,
    params?: Row.AsObject,
  }
}

export class Value extends jspb.Message {
  hasInteger(): boolean;
  clearInteger(): void;
  getInteger(): number;
  setInteger(value: number): void;

  hasText(): boolean;
  clearText(): void;
  getText(): string;
  setText(value: string): void;

  getTypeCase(): Value.TypeCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Value.AsObject;
  static toObject(includeInstance: boolean, msg: Value): Value.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Value, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Value;
  static deserializeBinaryFromReader(message: Value, reader: jspb.BinaryReader): Value;
}

export namespace Value {
  export type AsObject = {
    integer: number,
    text: string,
  }

  export enum TypeCase {
    TYPE_NOT_SET = 0,
    INTEGER = 1,
    TEXT = 2,
  }
}

export class Row extends jspb.Message {
  getColumnsMap(): jspb.Map<string, Value>;
  clearColumnsMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Row.AsObject;
  static toObject(includeInstance: boolean, msg: Row): Row.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Row, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Row;
  static deserializeBinaryFromReader(message: Row, reader: jspb.BinaryReader): Row;
}

export namespace Row {
  export type AsObject = {
    columnsMap: Array<[string, Value.AsObject]>,
  }
}

