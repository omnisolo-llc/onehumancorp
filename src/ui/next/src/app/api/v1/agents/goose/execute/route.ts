import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export async function POST(request: Request) {
  return proxyBackendRequest(request, "/api/v1/rpc", {
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("goose_mcp_execute", (input) => input),
  });
}
