import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const PROPOSAL_ID = /^[A-Za-z0-9._-]{1,128}$/;

export async function POST(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  const { id } = await context.params;
  if (!PROPOSAL_ID.test(id)) {
    return Response.json(
      { error: "invalid proposal ID" },
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
  return proxyBackendRequest(request, `/api/v1/proposals/${id}/approve`, {
    forwardQuery: false,
    suppressRequestBody: true,
  });
}
