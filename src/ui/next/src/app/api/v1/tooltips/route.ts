import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/tooltips");
}

export function POST(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/tooltips");
}

export function PUT(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/tooltips");
}
