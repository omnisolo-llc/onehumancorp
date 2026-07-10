import { proxyBackendGet, proxyBackendPost } from "../backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/pos/inventory");
}

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/pos/inventory");
}
