import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody(body) {
      const payload = JSON.parse(decoder.decode(body));
      if (typeof payload?.task !== "string" || payload.task.trim().length === 0) {
        throw new Error("invalid task");
      }
      return encoder.encode(JSON.stringify({
        jsonrpc: "2.0",
        id: crypto.randomUUID(),
        method: "run_deerflow_orchestration",
        params: { task: payload.task },
      }));
    },
  });
  if (!response.ok) return response;

  try {
    const payload = await response.json();
    if (payload?.error) {
      return Response.json({ error: payload.error.message ?? "Backend request failed" }, { status: 502 });
    }
    return Response.json({
      result: payload?.result?.output ?? payload?.result ?? "Executed successfully with empty output.",
    });
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}
