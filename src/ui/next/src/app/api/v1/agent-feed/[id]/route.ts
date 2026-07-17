import { proxyBackendPut } from "@/app/api/v1/ui/backendProxy";

const AGENT_FEED_ID = /^[A-Za-z0-9._-]{1,128}$/;

export async function PUT(req: Request, { params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  if (!AGENT_FEED_ID.test(id)) {
    return Response.json(
      { error: "invalid agent feed ID" },
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
  return proxyBackendPut(req, `/api/v1/agent-feed/${id}/state`);
}
