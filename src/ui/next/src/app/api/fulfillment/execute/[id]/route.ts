import { proxyBackendPost } from "../../../ui/backendProxy";
import { NextRequest } from "next/server";

export async function POST(req: NextRequest, context: { params: Promise<{ id: string }> }) {
  const p = await context.params;
  const id = p.id;
  return proxyBackendPost(req, `/api/fulfillment/execute/${id}`);
}
