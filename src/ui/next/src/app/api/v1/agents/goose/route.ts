import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

export async function GET(request: Request) {
  return proxyBackendRequest(request, "/api/v1/rpc", {
    backendMethod: "POST",
    requestContentType: "application/json",
    transformRequestBody: jsonRpcRequestTransform("goose_mcp_list", () => ({})),
  });
}
