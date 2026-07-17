import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export const runtime = "nodejs";

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("run_scalable_agents", (input) => ({
      count: input.count,
      message: input.message,
    })),
  });
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json(payload?.result ?? null);
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}
