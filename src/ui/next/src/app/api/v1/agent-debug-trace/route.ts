import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function GET(request: Request) {
  try {
    const res = await proxyBackendRequest(request, "/api/v1/agent-debug-trace", {
      suppressRequestBody: true,
    });
    if (res.ok) return res;
  } catch {
    // Fallback to empty trace array
  }
  return Response.json([]);
}
