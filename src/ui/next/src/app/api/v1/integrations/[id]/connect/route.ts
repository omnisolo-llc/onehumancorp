import { proxyBackendRequest } from "@/lib/auth/backendTransport";

type ConnectContext = { params: Promise<{ id: string }> };
const SAFE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

function credential(value: unknown): string | undefined {
  if (typeof value !== "string") return undefined;
  const trimmed = value.trim();
  return trimmed && trimmed.length <= 4096 ? trimmed : undefined;
}

export async function POST(request: Request, context: ConnectContext) {
  const { id } = await context.params;
  if (!SAFE_ID.test(id)) return Response.json({ error: "invalid integration" }, { status: 400 });
  return proxyBackendRequest(request, `/api/v1/integrations/${id}/connect`, {
    requestContentType: "application/json",
    transformRequestBody(body) {
      const payload: unknown = body.byteLength === 0 ? {} : JSON.parse(decoder.decode(body));
      if (payload === null || typeof payload !== "object" || Array.isArray(payload)) {
        throw new Error("expected credential object");
      }
      const record = payload as Record<string, unknown>;
      const outgoing: Record<string, string> = {};
      const botToken = credential(record.bot_token);
      const apiToken = credential(record.api_token);
      const fromPhone = credential(record.from_phone);
      if (botToken !== undefined) outgoing.bot_token = botToken;
      if (apiToken !== undefined) outgoing.api_token = apiToken;
      if (fromPhone !== undefined) outgoing.from_phone = fromPhone;
      return encoder.encode(JSON.stringify(outgoing));
    },
  });
}
