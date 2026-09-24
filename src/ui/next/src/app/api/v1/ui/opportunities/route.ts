import { proxyBackendGet } from "../backendProxy";

export async function GET(req: Request) {
  try {
    const res = await proxyBackendGet(req, "/api/v1/ui/opportunities");
    if (res.ok) return res;
  } catch {
    // Fallback to empty list
  }
  return Response.json([]);
}
