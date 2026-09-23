import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function GET(request: Request) {
  try {
    const res = await proxyBackendRequest(request, "/api/v1/ui/prep-forecast", {
      suppressRequestBody: true,
    });
    if (res.ok) return res;
  } catch {
    // Fallback to empty predictions list
  }
  return Response.json({ predictions: [] });
}
