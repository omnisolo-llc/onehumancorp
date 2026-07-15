import { proxyBackendGet, proxyBackendPost } from "../../ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/settings/sms-confirm");
}

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/v1/settings/sms-confirm");
}
