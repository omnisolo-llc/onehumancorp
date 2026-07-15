import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import {
  forgetMemoryRequest,
  invalidMemoryId,
  memoryId,
} from "../memoryPayload";

export const runtime = "nodejs";

export async function DELETE(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let id: string;
  try {
    id = memoryId((await context.params).id);
  } catch {
    return invalidMemoryId();
  }
  return proxyBackendRequest(request, "/api/v1/assistant/memory", {
    backendMethod: "PATCH",
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: forgetMemoryRequest(id),
  });
}
