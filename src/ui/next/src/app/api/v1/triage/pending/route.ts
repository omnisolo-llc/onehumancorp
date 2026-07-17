import { proxyBackendGet } from "@/app/api/v1/ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/triage/pending");
}
