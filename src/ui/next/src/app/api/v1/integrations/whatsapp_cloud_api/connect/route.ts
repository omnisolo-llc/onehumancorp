import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  const response = await proxyBackendRequest(
    request,
    "/api/v1/settings/integrations/whatsapp_cloud_api",
  );
  if (!response.ok) return response;
  return Response.json({ success: true, message: "WhatsApp Cloud API connected successfully" });
}
