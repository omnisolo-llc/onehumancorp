import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

export async function POST(request: Request) {
  return proxyBackendRequest(request, "/api/v1/growth/team-invites", {
    requestContentType: "application/json",
    transformRequestBody(body) {
      const payload = body.byteLength === 0 ? {} : JSON.parse(decoder.decode(body));
      return encoder.encode(JSON.stringify({ invitee_id: payload.invitee_id ?? "" }));
    },
  });
}
