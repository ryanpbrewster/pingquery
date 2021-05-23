// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_pb = require('./api_pb.js');

function serialize_pingquery_api_ExecRequest(arg) {
  if (!(arg instanceof api_pb.ExecRequest)) {
    throw new Error('Expected argument of type pingquery.api.ExecRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_ExecRequest(buffer_arg) {
  return api_pb.ExecRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_pingquery_api_ExecResponse(arg) {
  if (!(arg instanceof api_pb.ExecResponse)) {
    throw new Error('Expected argument of type pingquery.api.ExecResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_ExecResponse(buffer_arg) {
  return api_pb.ExecResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_pingquery_api_GetConfigRequest(arg) {
  if (!(arg instanceof api_pb.GetConfigRequest)) {
    throw new Error('Expected argument of type pingquery.api.GetConfigRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_GetConfigRequest(buffer_arg) {
  return api_pb.GetConfigRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_pingquery_api_GetConfigResponse(arg) {
  if (!(arg instanceof api_pb.GetConfigResponse)) {
    throw new Error('Expected argument of type pingquery.api.GetConfigResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_GetConfigResponse(buffer_arg) {
  return api_pb.GetConfigResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_pingquery_api_InteractRequest(arg) {
  if (!(arg instanceof api_pb.InteractRequest)) {
    throw new Error('Expected argument of type pingquery.api.InteractRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_InteractRequest(buffer_arg) {
  return api_pb.InteractRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_pingquery_api_InteractResponse(arg) {
  if (!(arg instanceof api_pb.InteractResponse)) {
    throw new Error('Expected argument of type pingquery.api.InteractResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_InteractResponse(buffer_arg) {
  return api_pb.InteractResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_pingquery_api_SetConfigRequest(arg) {
  if (!(arg instanceof api_pb.SetConfigRequest)) {
    throw new Error('Expected argument of type pingquery.api.SetConfigRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_SetConfigRequest(buffer_arg) {
  return api_pb.SetConfigRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_pingquery_api_SetConfigResponse(arg) {
  if (!(arg instanceof api_pb.SetConfigResponse)) {
    throw new Error('Expected argument of type pingquery.api.SetConfigResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_pingquery_api_SetConfigResponse(buffer_arg) {
  return api_pb.SetConfigResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var PingQueryService = exports.PingQueryService = {
  getConfig: {
    path: '/pingquery.api.PingQuery/GetConfig',
    requestStream: false,
    responseStream: false,
    requestType: api_pb.GetConfigRequest,
    responseType: api_pb.GetConfigResponse,
    requestSerialize: serialize_pingquery_api_GetConfigRequest,
    requestDeserialize: deserialize_pingquery_api_GetConfigRequest,
    responseSerialize: serialize_pingquery_api_GetConfigResponse,
    responseDeserialize: deserialize_pingquery_api_GetConfigResponse,
  },
  setConfig: {
    path: '/pingquery.api.PingQuery/SetConfig',
    requestStream: false,
    responseStream: false,
    requestType: api_pb.SetConfigRequest,
    responseType: api_pb.SetConfigResponse,
    requestSerialize: serialize_pingquery_api_SetConfigRequest,
    requestDeserialize: deserialize_pingquery_api_SetConfigRequest,
    responseSerialize: serialize_pingquery_api_SetConfigResponse,
    responseDeserialize: deserialize_pingquery_api_SetConfigResponse,
  },
  exec: {
    path: '/pingquery.api.PingQuery/Exec',
    requestStream: false,
    responseStream: false,
    requestType: api_pb.ExecRequest,
    responseType: api_pb.ExecResponse,
    requestSerialize: serialize_pingquery_api_ExecRequest,
    requestDeserialize: deserialize_pingquery_api_ExecRequest,
    responseSerialize: serialize_pingquery_api_ExecResponse,
    responseDeserialize: deserialize_pingquery_api_ExecResponse,
  },
  interact: {
    path: '/pingquery.api.PingQuery/Interact',
    requestStream: true,
    responseStream: true,
    requestType: api_pb.InteractRequest,
    responseType: api_pb.InteractResponse,
    requestSerialize: serialize_pingquery_api_InteractRequest,
    requestDeserialize: deserialize_pingquery_api_InteractRequest,
    responseSerialize: serialize_pingquery_api_InteractResponse,
    responseDeserialize: deserialize_pingquery_api_InteractResponse,
  },
};

exports.PingQueryClient = grpc.makeGenericClientConstructor(PingQueryService);
