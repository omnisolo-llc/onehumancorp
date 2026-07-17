import { proxyBackendGet } from "../backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/ui/opportunities");
}
