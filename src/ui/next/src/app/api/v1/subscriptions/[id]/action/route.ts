import {
  validateJsonRequestBody,
  proxyBackendRequest,
} from "@/lib/auth/backendTransport";
import {
  invalidSubscriptionId,
  subscriptionBackendPath,
} from "../../subscriptionBackend";

export async function POST(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = subscriptionBackendPath((await context.params).id, "/action");
  } catch {
    return invalidSubscriptionId();
  }
  return proxyBackendRequest(request, path, {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
