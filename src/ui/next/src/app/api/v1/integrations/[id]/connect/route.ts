import { proxyBackendRequest } from "@/lib/auth/backendTransport";

type ConnectContext = { params: Promise<{ id: string }> };
const SAFE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

export async function POST(request: Request, context: ConnectContext) {
  const { id } = await context.params;
  if (!SAFE_ID.test(id)) return Response.json({ error: "invalid integration" }, { status: 400 });
  const targetId = id === "whatsapp" ? "twilio" : id;
  return proxyBackendRequest(request, `/api/v1/integrations/${targetId}/connect`, {
    requestContentType: "application/json",
    transformRequestBody(body) {
      const payload = body.byteLength === 0 ? {} : JSON.parse(decoder.decode(body));
      return encoder.encode(JSON.stringify({ integration_id: id, ...payload }));
    },
  });
}
