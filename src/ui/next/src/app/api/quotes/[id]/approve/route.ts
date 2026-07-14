import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { invalidQuoteId, quoteBackendPath } from "../../quoteBackend";

export async function PATCH(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = quoteBackendPath((await context.params).id, "/approve");
  } catch {
    return invalidQuoteId();
  }
  return proxyBackendRequest(request, path, {
    backendMethod: "PATCH",
    forwardQuery: false,
    requestContentType: "application/json",
    suppressRequestBody: true,
  });
}
