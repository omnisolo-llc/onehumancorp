import { proxyBackendPost } from "../../../ui/backendProxy";

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/agents/approvals/simulate-dispute-resolution");
}
