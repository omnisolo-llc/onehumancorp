import {
  validateJsonRequestBody,
  proxyBackendRequest,
} from "@/lib/auth/backendTransport";

const FULFILLMENT_ID = /^[A-Za-z0-9._-]{1,128}$/;

function invalidFulfillmentId(): Response {
  return Response.json(
    { error: "invalid fulfillment ID" },
    {
      status: 400,
      headers: {
        "cache-control": "private, no-store",
        pragma: "no-cache",
        "x-content-type-options": "nosniff",
      },
    },
  );
}

export async function POST(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  const { id } = await context.params;
  if (id === "." || id === ".." || !FULFILLMENT_ID.test(id)) {
    return invalidFulfillmentId();
  }
  return proxyBackendRequest(request, `/api/fulfillment/execute/${id}`, {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
}
