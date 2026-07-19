import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  const response = await proxyBackendRequest(request, "/api/v1/settings/integrations/whatsapp");
  if (!response.ok) return response;
  return Response.json({ success: true, message: "Twilio WhatsApp connected successfully" });
}
