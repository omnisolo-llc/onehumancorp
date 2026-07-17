import { proxyBackendGet, proxyBackendPost } from "../backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/pos/inventory");
}

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/v1/pos/inventory");
}
