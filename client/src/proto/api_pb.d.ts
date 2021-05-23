// package: pingquery.api
// file: api.proto

import * as jspb from "google-protobuf";

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
  }
}

export class Config extends jspb.Message {
  clearQueriesList(): void;
  getQueriesList(): Array<StatementConfig>;
  setQueriesList(value: Array<StatementConfig>): void;
  addQueries(value?: StatementConfig, index?: number): StatementConfig;

  clearMutatesList(): void;
  getMutatesList(): Array<StatementConfig>;
  setMutatesList(value: Array<StatementConfig>): void;
  addMutates(value?: StatementConfig, index?: number): StatementConfig;

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
    queriesList: Array<StatementConfig.AsObject>,
    mutatesList: Array<StatementConfig.AsObject>,
  }
}

export class StatementConfig extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getSqlTemplate(): string;
  setSqlTemplate(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StatementConfig.AsObject;
  static toObject(includeInstance: boolean, msg: StatementConfig): StatementConfig.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StatementConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StatementConfig;
  static deserializeBinaryFromReader(message: StatementConfig, reader: jspb.BinaryReader): StatementConfig;
}

export namespace StatementConfig {
  export type AsObject = {
    name: string,
    sqlTemplate: string,
  }
}

export class Statement extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getParamsMap(): jspb.Map<string, Value>;
  clearParamsMap(): void;
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
    paramsMap: Array<[string, Value.AsObject]>,
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

