import {
  proxyBackendRequest,
  stripBrowserIdentityJsonRequestBody,
} from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export const runtime = "nodejs";

const ALLOWED_METHODS = new Set([
  "ap_create_task",
  "ap_execute_step",
  "ap_list_checkpoints",
  "ap_list_steps",
  "ap_list_tasks",
  "ap_restore_checkpoint",
]);
const READ_METHODS = new Set(["ap_list_tasks", "ap_list_steps", "ap_list_checkpoints"]);
const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

async function unwrapResult(response: Response): Promise<Response> {
  if (!response.ok) return response;
  const json = (body: unknown, status = 200) => Response.json(body, {
    status, headers: { "Cache-Control": "private, no-store" },
  });
  const invalid = () => json({ error: "Backend returned an invalid Agent Protocol response" }, 502);
  if (response.status !== 200) return invalid();
  try {
    const payload = await response.json();
    if (!payload || typeof payload !== "object" || Array.isArray(payload)) return invalid();
    const result = payload.result;
    // The runtime reports transport-level RPC errors and operation-level errors
    // (for example, an unconfigured checkpointer) in different envelopes.
    const error = payload.error ?? result?.error;
    if (error != null) {
      const message = typeof error === "string" ? error : error.message;
      return json({ error: typeof message === "string" && message.trim() ? message : "Agent Protocol operation failed" }, 502);
    }
    if (!result || typeof result !== "object" || Array.isArray(result)
      || payload.success !== undefined && payload.success !== true
      || result.success !== undefined && result.success !== true) return invalid();
    return json(result);
  } catch {
    return invalid();
  }
}

export async function GET(request: Request) {
  const url = new URL(request.url);
  const method = url.searchParams.get("method") ?? "";
  if (!ALLOWED_METHODS.has(method)) {
    return Response.json({ error: "unsupported method" }, { status: 400 });
  }
  if (!READ_METHODS.has(method)) {
    return Response.json({ error: "This operation requires POST" }, { status: 405, headers: { Allow: "POST" } });
  }
  const taskId = url.searchParams.get("task_id");
  return unwrapResult(await proxyBackendRequest(request, "/api/v1/rpc", {
    backendMethod: "POST",
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform(method, () => (
      taskId === null ? {} : { task_id: taskId }
    )),
  }));
}

export async function POST(request: Request) {
  return unwrapResult(await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody(body) {
      const input = JSON.parse(
        decoder.decode(stripBrowserIdentityJsonRequestBody(body)),
      ) as Record<string, unknown>;
      if (typeof input.method !== "string" || !ALLOWED_METHODS.has(input.method)) {
        throw new Error("unsupported method");
      }
      const params = input.params;
      if (params !== undefined && (params === null || typeof params !== "object" || Array.isArray(params))) {
        throw new Error("invalid params");
      }
      return encoder.encode(JSON.stringify({
        jsonrpc: "2.0",
        id: crypto.randomUUID(),
        method: input.method,
        params: params ?? {},
      }));
    },
  }));
}
