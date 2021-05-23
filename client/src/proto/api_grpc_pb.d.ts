// GENERATED CODE -- DO NOT EDIT!

// package: pingquery.api
// file: api.proto

import * as api_pb from "./api_pb";
import * as grpc from "@grpc/grpc-js";

interface IPingQueryService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  getConfig: grpc.MethodDefinition<api_pb.GetConfigRequest, api_pb.GetConfigResponse>;
  setConfig: grpc.MethodDefinition<api_pb.SetConfigRequest, api_pb.SetConfigResponse>;
  exec: grpc.MethodDefinition<api_pb.ExecRequest, api_pb.ExecResponse>;
  interact: grpc.MethodDefinition<api_pb.InteractRequest, api_pb.InteractResponse>;
}

export const PingQueryService: IPingQueryService;

export interface IPingQueryServer extends grpc.UntypedServiceImplementation {
  getConfig: grpc.handleUnaryCall<api_pb.GetConfigRequest, api_pb.GetConfigResponse>;
  setConfig: grpc.handleUnaryCall<api_pb.SetConfigRequest, api_pb.SetConfigResponse>;
  exec: grpc.handleUnaryCall<api_pb.ExecRequest, api_pb.ExecResponse>;
  interact: grpc.handleBidiStreamingCall<api_pb.InteractRequest, api_pb.InteractResponse>;
}

export class PingQueryClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  getConfig(argument: api_pb.GetConfigRequest, callback: grpc.requestCallback<api_pb.GetConfigResponse>): grpc.ClientUnaryCall;
  getConfig(argument: api_pb.GetConfigRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_pb.GetConfigResponse>): grpc.ClientUnaryCall;
  getConfig(argument: api_pb.GetConfigRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_pb.GetConfigResponse>): grpc.ClientUnaryCall;
  setConfig(argument: api_pb.SetConfigRequest, callback: grpc.requestCallback<api_pb.SetConfigResponse>): grpc.ClientUnaryCall;
  setConfig(argument: api_pb.SetConfigRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_pb.SetConfigResponse>): grpc.ClientUnaryCall;
  setConfig(argument: api_pb.SetConfigRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_pb.SetConfigResponse>): grpc.ClientUnaryCall;
  exec(argument: api_pb.ExecRequest, callback: grpc.requestCallback<api_pb.ExecResponse>): grpc.ClientUnaryCall;
  exec(argument: api_pb.ExecRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_pb.ExecResponse>): grpc.ClientUnaryCall;
  exec(argument: api_pb.ExecRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_pb.ExecResponse>): grpc.ClientUnaryCall;
  interact(metadataOrOptions?: grpc.Metadata | grpc.CallOptions | null): grpc.ClientDuplexStream<api_pb.InteractRequest, api_pb.InteractResponse>;
  interact(metadata?: grpc.Metadata | null, options?: grpc.CallOptions | null): grpc.ClientDuplexStream<api_pb.InteractRequest, api_pb.InteractResponse>;
}
