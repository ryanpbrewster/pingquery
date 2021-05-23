const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');
const path = require("path");

const PROTO_PATH = path.join(__dirname, '../../proto/api.proto');
// Suggested options for similarity to existing grpc.load behavior
const packageDefinition = protoLoader.loadSync(PROTO_PATH);
const protoDescriptor = grpc.loadPackageDefinition(packageDefinition);
const api = protoDescriptor.pingquery.api;

const client = new api.PingQuery('localhost:50051', grpc.credentials.createInsecure());

async function main() {
  await new Promise((resolve, reject) => {
    client.getConfig({}, (err, res) => {
      return err ? reject(err) : resolve(res);
    });
  });
}

main().catch((err) => console.error(err));
