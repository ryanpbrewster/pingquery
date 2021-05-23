import { PingQueryClient } from "./proto/api_grpc_pb";
import { credentials } from "@grpc/grpc-js";
import {
  ExecRequest,
  ExecResponse,
  GetConfigRequest,
  GetConfigResponse,
  SetConfigRequest,
  SetConfigResponse,
} from "./proto/api_pb";

export default class Client {
  private readonly inner: PingQueryClient;
  constructor(address: string) {
    this.inner = new PingQueryClient(address, credentials.createInsecure());
  }

  async getConfig(request: GetConfigRequest): Promise<GetConfigResponse> {
    return new Promise((resolve, reject) =>
      this.inner.getConfig(request, (err, resp) =>
        resp ? resolve(resp) : reject(err)
      )
    );
  }

  async setConfig(request: SetConfigRequest): Promise<SetConfigResponse> {
    return new Promise((resolve, reject) =>
      this.inner.setConfig(request, (err, resp) =>
        resp ? resolve(resp) : reject(err)
      )
    );
  }

  async exec(request: ExecRequest): Promise<ExecResponse> {
    return new Promise((resolve, reject) =>
      this.inner.exec(request, (err, resp) =>
        resp ? resolve(resp) : reject(err)
      )
    );
  }
}
