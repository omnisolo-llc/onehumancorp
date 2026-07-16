import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import {
  privateJson,
  taskBackendPath,
  withTaskMutationEnvelope,
} from "../../assistantBackend";

export async function PATCH(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = taskBackendPath((await context.params).id);
  } catch {
    return privateJson(400, { error: "invalid task ID" });
  }
  return withTaskMutationEnvelope(proxyBackendRequest(request, path));
}
