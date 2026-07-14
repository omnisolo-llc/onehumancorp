import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { withSuccessStatus } from "../assistantBackend";

export function GET(request: Request): Promise<Response> {
  return proxyBackendRequest(request, "/api/assistant/tasks");
}

export function POST(request: Request): Promise<Response> {
  return withSuccessStatus(
    proxyBackendRequest(request, "/api/assistant/tasks"),
    201,
  );
}
