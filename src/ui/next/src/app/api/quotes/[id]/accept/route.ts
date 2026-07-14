import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { invalidQuoteId, quoteBackendPath } from "../../quoteBackend";

export async function POST(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = quoteBackendPath((await context.params).id, "/accept");
  } catch {
    return invalidQuoteId();
  }
  return proxyBackendRequest(request, path);
}
