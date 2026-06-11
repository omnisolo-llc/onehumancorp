import { proxyBackendPost } from "../../../backendProxy";

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/ui/triage/action");
}
