import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";
import { FaultInjector } from "@/lib/chaos";

export async function POST(request: Request) {
  try {
    await FaultInjector.applyFault("ralph_loop_api_start");
    await FaultInjector.applyFault("ralph_loop_api_fetch_before");
    const response = await proxyBackendRequest(request, "/api/v1/rpc", {
      requestContentType: "application/json",
      transformRequestBody: jsonRpcRequestTransform("run_ralph_loop", (input) => {
        if (typeof input.task !== "string" || input.task.trim().length === 0) {
          throw new Error("task is required");
        }
        return {
          task: input.task,
          progress_file: typeof input.progress_file === "string"
            ? input.progress_file
            : ".ralph_progress.json",
        };
      }),
    });
    await FaultInjector.applyFault("ralph_loop_api_fetch_after");
    if (!response.ok) return response;
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json({ result: payload?.result });
  } catch (error) {
    const message = error instanceof Error ? error.message : "invalid request";
    if (message.includes("Fault Injected")) {
      return Response.json({ error: `Backend service unavailable: ${message}` }, { status: 503 });
    }
    return Response.json({ error: message }, { status: 400 });
  }
}
