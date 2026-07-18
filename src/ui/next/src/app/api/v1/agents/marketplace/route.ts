import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

async function unwrapResult(response: Response): Promise<Response> {
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json(payload?.result ?? null);
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}

export async function GET(request: Request) {
  const url = new URL(request.url);
  const fetchOne = url.searchParams.get("method") === "fetch";
  const transform = fetchOne
    ? jsonRpcRequestTransform("am_fetch_agent", () => ({
        agent_id: url.searchParams.get("agent_id"),
      }))
    : jsonRpcRequestTransform("am_search_agents", () => ({
        query: url.searchParams.get("q") ?? "",
      }));
  return unwrapResult(await proxyBackendRequest(request, "/api/v1/rpc", {
    backendMethod: "POST",
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: transform,
  }));
}

export async function POST(request: Request) {
  return unwrapResult(await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("am_publish_agent", (input) => input),
  }));
}
