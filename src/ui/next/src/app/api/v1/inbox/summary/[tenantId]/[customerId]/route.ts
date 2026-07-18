import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { memoryId } from "../../../../memory/memoryPayload";

export const runtime = "nodejs";

export async function GET(
  request: Request,
  context: { params: Promise<{ tenantId: string, customerId: string }> },
): Promise<Response> {
  let customerId = "";
  try {
    customerId = memoryId((await context.params).customerId);
  } catch (e) {
    return Response.json({ error: "invalid memory id" }, { status: 400 });
  }

  const path = `/api/v1/memory/summary/${customerId}`;
  return proxyBackendRequest(request, path, {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}
