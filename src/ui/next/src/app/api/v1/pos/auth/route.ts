import { proxyBackendPost } from "@/app/api/v1/ui/backendProxy";

export function POST(request: Request): Promise<Response> {
  return proxyBackendPost(request, "/api/v1/pos/auth");
}
