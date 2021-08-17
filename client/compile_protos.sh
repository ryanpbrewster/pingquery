#!/bin/sh
set -eux

OUT_DIR="src/proto"
TS_OUT_DIR="src/proto"
IN_DIR="../server/proto"
PROTOC="$(npm bin)/grpc_tools_node_protoc"
PROTOC_GEN_TS_PATH="$(npm bin)/protoc-gen-ts_proto"

$PROTOC \
    -I="../server/proto" \
    --plugin=protoc-gen-ts_proto=$PROTOC_GEN_TS_PATH \
    --js_out=import_style=commonjs:$OUT_DIR \
    --ts_proto_out=$TS_OUT_DIR \
    "$IN_DIR"/*.proto
