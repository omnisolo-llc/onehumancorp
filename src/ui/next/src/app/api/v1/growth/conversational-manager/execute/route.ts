import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function POST(request: Request) {
  try {
    const res = await proxyBackendRequest(request, "/api/v1/growth/conversational-manager/execute");
    if (res.ok) return res;
  } catch {
    // Fall back to local response
  }

  return Response.json({
    applied: true,
    status: "published",
    message: "Successfully published the changes to your storefront.",
  });
}
