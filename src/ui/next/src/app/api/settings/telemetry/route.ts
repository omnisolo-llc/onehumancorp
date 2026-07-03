import { proxyBackendGet, proxyBackendPost } from "../../ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/settings/telemetry");
}

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/settings/telemetry");
}
