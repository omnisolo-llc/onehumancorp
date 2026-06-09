import { proxyBackendPost } from "../../../backendProxy";
import { NextRequest } from "next/server";

export async function POST(req: NextRequest, { params }: { params: { id: string } }) {
  const body = await req.text();
  const forwardReq = new Request(req.url, {
    method: "POST",
    headers: req.headers,
    body: body || undefined,
  });
  return proxyBackendPost(forwardReq, `/api/ui/triage/${params.id}/approve`);
}
