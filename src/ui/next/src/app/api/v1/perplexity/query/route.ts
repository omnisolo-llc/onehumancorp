import {
  proxyBackendRequest,
  stripBrowserIdentityJsonRequestBody,
} from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  return proxyBackendRequest(request, "/api/v1/perplexity/query", {
    requestContentType: "application/json",
    transformRequestBody: stripBrowserIdentityJsonRequestBody,
  });
}
