import { proxyBackendPut } from "@/app/api/v1/ui/backendProxy";
import { NextRequest } from "next/server";

const AGENT_FEED_ID = /^[A-Za-z0-9._-]{1,128}$/;

export async function PUT(req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  if (!AGENT_FEED_ID.test(id)) {
    return Response.json({ error: "invalid agent feed ID" }, { status: 400 });
  }
  return proxyBackendPut(req, `/api/v1/agent-feed/${id}/state`);
}
