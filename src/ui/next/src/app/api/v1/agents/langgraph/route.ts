import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("run_agent", (input) => {
      if (typeof input.message !== "string" || input.message.trim().length === 0) {
        throw new Error("message is required");
      }
      return {
        agent_id: "default",
        message: input.message,
        config: { enable_langgraph_mechanic: true },
      };
    }),
  });
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json({ result: payload?.result });
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}
