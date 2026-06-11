import { proxyBackendPost } from "../../../../../ui/backendProxy";
export async function POST(request: Request) {
  return proxyBackendPost(request, "/api/v1/payments/terminal/session/end");
}
