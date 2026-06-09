import { proxyBackendPost } from "../../../backendProxy";

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/ui/dashboard/analytics/chat");
}
