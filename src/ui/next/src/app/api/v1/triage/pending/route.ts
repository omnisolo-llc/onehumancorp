import { proxyBackendGet } from "@/app/api/ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/triage/pending");
}
