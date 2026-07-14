import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function proxyCurrentBackendPath(request: Request): Promise<Response> {
  return proxyBackendRequest(request, new URL(request.url).pathname);
}
