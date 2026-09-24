import { proxyBackendGet } from "../backendProxy";

export const runtime = "nodejs";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/ui/inbox");
}
