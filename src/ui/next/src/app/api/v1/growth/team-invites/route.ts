import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function GET(request: Request) {
  return proxyBackendRequest(request, "/api/v1/growth/team-invites", {
    suppressRequestBody: true,
  });
}

export async function POST(request: Request) {
  return proxyBackendRequest(request, "/api/v1/growth/team-invites");
}
