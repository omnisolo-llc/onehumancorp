import { proxyBackendRequest, validateJsonRequestBody } from "@/lib/auth/backendTransport";
import { invalidQuoteId, quoteBackendPath } from "../quoteBackend";

export async function GET(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = quoteBackendPath((await context.params).id);
  } catch {
    return invalidQuoteId();
  }
  return proxyBackendRequest(request, path, {
    forwardQuery: false,
    requestContentType: "application/json",
  });
}

export async function PUT(
  request: Request,
  context: { params: Promise<{ id: string }> | { id: string } },
): Promise<Response> {
  let path: string;
  try {
    path = quoteBackendPath((await context.params).id);
  } catch {
    return invalidQuoteId();
  }
  return proxyBackendRequest(request, path, {
    backendMethod: "PUT",
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
