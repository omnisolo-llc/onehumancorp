import { runtimeTextResponse } from "@/lib/auth/runtimeResponse";
import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("run_actor_model", (input) => {
      if (typeof input.message !== "string" || input.message.trim().length === 0) {
        throw new Error("message is required");
      }
      return { message: input.message };
    }),
  });
  return runtimeTextResponse(response);
}
