import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { taskBackendPathFromBody, withSuccessStatus } from "../assistantBackend";

export function POST(request: Request): Promise<Response> {
  return withSuccessStatus(
    proxyBackendRequest(request, "/api/v1/assistant/tasks/invalid/artifacts", {
      resolveBackendPath: taskBackendPathFromBody("/artifacts"),
    }),
    201,
  );
}
