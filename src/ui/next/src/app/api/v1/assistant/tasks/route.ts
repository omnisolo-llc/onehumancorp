import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { withSuccessStatus } from "../assistantBackend";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/v1/assistant/tasks");
}

export function POST(request: Request): Promise<Response> {
  return withSuccessStatus(
    proxyBackendRequest(request, "/api/v1/assistant/tasks"),
    201,
  );
}
