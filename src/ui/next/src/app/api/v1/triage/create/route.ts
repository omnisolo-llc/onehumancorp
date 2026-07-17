import { proxyBackendPost } from "@/app/api/v1/ui/backendProxy";

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/v1/triage/create");
}
