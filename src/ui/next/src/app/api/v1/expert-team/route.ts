import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";
import { FaultInjector } from "@/lib/chaos";

export async function POST(request: Request) {
  try {
    await FaultInjector.applyFault("expert_team_api_start");
    await FaultInjector.applyFault("expert_team_api_fetch_before");
    const response = await proxyBackendRequest(request, "/api/v1/rpc", {
      requestContentType: "application/json",
      transformRequestBody: jsonRpcRequestTransform("run_expert_team", (input) => {
        if (typeof input.task !== "string" || input.task.trim().length === 0) {
          throw new Error("task is required");
        }
        return { message: input.task };
      }),
    });
    await FaultInjector.applyFault("expert_team_api_fetch_after");
    if (!response.ok) return response;
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json({
      result: payload?.result?.output ?? "Executed successfully via Expert Team.",
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : "invalid request";
    if (message.includes("expert_team_api_start")) {
      return Response.json({ error: message }, { status: 500 });
    }
    if (message.includes("Fault Injected")) {
      return Response.json({ error: "Backend service unavailable" }, { status: 503 });
    }
    return Response.json({ error: message }, { status: 400 });
  }
}
