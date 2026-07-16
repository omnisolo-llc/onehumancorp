import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { invalidMemoryId, memoryCustomerPath } from "../../memoryPayload";

export const runtime = "nodejs";

export async function GET(
  request: Request,
  context: { params: Promise<{ customerId: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = memoryCustomerPath((await context.params).customerId);
  } catch {
    return invalidMemoryId();
  }
  return proxyBackendRequest(request, path, {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}
