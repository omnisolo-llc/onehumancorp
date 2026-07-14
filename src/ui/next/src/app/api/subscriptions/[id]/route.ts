import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import {
  invalidSubscriptionId,
  subscriptionBackendPath,
} from "../subscriptionBackend";

export async function GET(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = subscriptionBackendPath((await context.params).id);
  } catch {
    return invalidSubscriptionId();
  }
  return proxyBackendRequest(request, path, { forwardQuery: false });
}
