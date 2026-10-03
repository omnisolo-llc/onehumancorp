import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { safeReturnPath } from "@/lib/auth/url";

export async function POST(request: Request) {
  return proxyBackendRequest(request, "/api/v1/growth/referrals/click");
}

export async function GET(request: Request) {
  const url = new URL(request.url);
  const target = safeReturnPath(url.searchParams.get("target") ?? "/onboarding");
  const ref = url.searchParams.get("ref");
  if (ref) {
    await proxyBackendRequest(request, "/api/v1/growth/referrals/click", {
      suppressRequestBody: true,
    });
  }
  const host = request.headers.get("x-forwarded-host") || request.headers.get("host");
  const proto = request.headers.get("x-forwarded-proto") || (url.protocol.replace(":", "") || "http");
  const baseOrigin = host ? `${proto}://${host}` : (process.env.OMNISOLO_WEB_CANONICAL_ORIGIN || url.origin);
  const redirect = new URL(target, baseOrigin);
  if (ref) redirect.searchParams.set("ref", ref);
  return Response.redirect(redirect);
}
