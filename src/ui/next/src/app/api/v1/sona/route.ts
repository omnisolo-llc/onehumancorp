import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export async function GET(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/rpc", {
    backendMethod: "POST",
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("get_sona_patterns", () => ({})),
  });
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json({ patterns: payload?.result?.patterns ?? [] });
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("record_sona_pattern", (input) => input),
  });
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json({ status: "success" });
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}
