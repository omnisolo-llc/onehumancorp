import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const WALKTHROUGH_PAGE = /^[A-Za-z0-9_-]{1,128}$/;

export async function GET(
  request: Request,
  context: { params: Promise<{ page: string }> },
): Promise<Response> {
  const page = (await context.params).page;
  if (!WALKTHROUGH_PAGE.test(page)) {
    return Response.json(
      { error: "invalid walkthrough page" },
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
  return proxyBackendRequest(request, `/api/v1/walkthrough/${page}`);
}
