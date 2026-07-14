import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { approvalBackendPath, privateApprovalError } from "../approvalBackend";

export async function POST(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = approvalBackendPath((await context.params).id);
  } catch {
    return privateApprovalError(400, "invalid approval ID");
  }
  return proxyBackendRequest(request, path);
}
