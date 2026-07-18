import {
  proxyBackendRequest,
  stripBrowserIdentityJsonRequestBody,
} from "@/lib/auth/backendTransport";

export function proxyCurrentBackendPath(request: Request): Promise<Response> {
  const contentType = request.headers.get("content-type")?.split(";", 1)[0].trim().toLowerCase();
  const path = new URL(request.url).pathname;
  if (contentType === "application/json") {
    return proxyBackendRequest(request, path, {
      transformRequestBody: stripBrowserIdentityJsonRequestBody,
    });
  }
  return proxyBackendRequest(request, path);
}
