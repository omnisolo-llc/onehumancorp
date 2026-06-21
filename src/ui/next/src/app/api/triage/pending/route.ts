import { proxyBackendGet } from "../../ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/ui/unified_triage/pending");
}
