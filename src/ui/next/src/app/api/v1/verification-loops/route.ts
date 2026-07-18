import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("verify_output", (input) => {
      if (
        typeof input.output_text !== "string" || input.output_text.length === 0 ||
        typeof input.verification_type !== "string" || input.verification_type.length === 0
      ) {
        throw new Error("output_text and verification_type are required");
      }
      return {
        output_text: input.output_text,
        task_context: typeof input.task_context === "string" ? input.task_context : "",
        verification_type: input.verification_type,
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
