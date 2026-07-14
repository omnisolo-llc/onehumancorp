import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import {
  invalidQuoteId,
  normalizeLegacyQuoteBody,
  quoteBackendPath,
  quoteIdFromUrl,
} from "./quoteBackend";

function pathFromRequest(request: Request): string | Response {
  try {
    const id = quoteIdFromUrl(request.url);
    return id === null ? "/api/v1/quotes" : quoteBackendPath(id);
  } catch {
    return invalidQuoteId();
  }
}

export function GET(request: Request): Promise<Response> | Response {
  const path = pathFromRequest(request);
  return typeof path === "string"
    ? proxyBackendRequest(request, path, {
        forwardQuery: false,
        requestContentType: "application/json",
      })
    : path;
}

export function POST(request: Request): Promise<Response> | Response {
  const path = pathFromRequest(request);
  if (typeof path !== "string") return path;
  const updating = path !== "/api/v1/quotes";
  return proxyBackendRequest(request, path, {
    backendMethod: updating ? "PUT" : "POST",
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: normalizeLegacyQuoteBody,
  });
}
