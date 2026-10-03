import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request): Promise<Response> {
  try {
    return await proxyBackendRequest(request, "/api/v1/agent/draft-escalation");
  } catch {
    return Response.json(
      { error: "Escalation draft is unavailable" },
      { status: 502, headers: { "cache-control": "private, no-store" } },
    );
  }
}
