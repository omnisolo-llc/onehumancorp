import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function proxyBackendGet(request: Request, backendPath: string): Promise<Response> {
  return proxyBackendRequest(request, backendPath);
}

export function proxyBackendPost(request: Request, backendPath: string): Promise<Response> {
  return proxyBackendRequest(request, backendPath);
}

export function proxyBackendPatch(request: Request, backendPath: string): Promise<Response> {
  return proxyBackendRequest(request, backendPath);
}

export function proxyBackendPut(request: Request, backendPath: string): Promise<Response> {
  return proxyBackendRequest(request, backendPath);
}
