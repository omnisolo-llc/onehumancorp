import { proxyBackendGet } from "../../ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/advisory/insights");
}
