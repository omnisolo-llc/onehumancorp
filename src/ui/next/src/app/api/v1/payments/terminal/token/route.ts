import { proxyBackendRequest } from "@/lib/auth/backendTransport";

function privateJson(status: number, value: unknown): Response {
  return Response.json(value, {
    status,
    headers: {
      "cache-control": "private, no-store",
      pragma: "no-cache",
      "x-content-type-options": "nosniff",
    },
  });
}

function terminalToken(payload: unknown): string | undefined {
  if (payload === null || typeof payload !== "object" || Array.isArray(payload)) return;
  const value = payload as Record<string, unknown>;
  if (typeof value.secret === "string") return value.secret;
  if (typeof value.token === "string") return value.token;
  if (value.Ok !== null && typeof value.Ok === "object" && !Array.isArray(value.Ok)) {
    const ok = value.Ok as Record<string, unknown>;
    if (typeof ok.secret === "string") return ok.secret;
    if (typeof ok.token === "string") return ok.token;
  }
}

export async function POST(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, "/api/v1/payments/terminal/token");
  if (!response.ok) return response;
  const payload = await response.json().catch(() => null);
  const secret = terminalToken(payload);
  return secret === undefined
    ? privateJson(502, { error: "Backend response did not include a Terminal secret" })
    : privateJson(200, { secret });
}
