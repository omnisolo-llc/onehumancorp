import { proxyBackendGet, proxyBackendPost } from "../ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/agent-feed");
}

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/agent-feed");
}
