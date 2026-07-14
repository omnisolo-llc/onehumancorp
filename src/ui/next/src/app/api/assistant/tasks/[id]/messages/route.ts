import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import {
  privateJson,
  taskBackendPath,
  withSuccessStatus,
} from "../../../assistantBackend";

export async function POST(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = taskBackendPath((await context.params).id, "/messages");
  } catch {
    return privateJson(400, { error: "invalid task ID" });
  }
  return withSuccessStatus(proxyBackendRequest(request, path), 201);
}
