import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { invalidProposalId, proposalBackendPath } from "../../proposalBackend";

export async function POST(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = proposalBackendPath((await context.params).id, "/accept");
  } catch {
    return invalidProposalId();
  }
  return proxyBackendRequest(request, path, {
    forwardQuery: false,
    requestContentType: "application/json",
    suppressRequestBody: true,
  });
}
