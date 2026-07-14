import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { taskBackendPathFromBody, withSuccessStatus } from "../assistantBackend";

export function POST(request: Request): Promise<Response> {
  return withSuccessStatus(
    proxyBackendRequest(request, "/unused", {
      resolveBackendPath: taskBackendPathFromBody("/artifacts"),
    }),
    201,
  );
}
