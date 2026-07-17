import {
  proxyBackendRequest,
  stripBrowserIdentityJsonRequestBody,
} from "@/lib/auth/backendTransport";

type RouteContext = { params: Promise<{ path: string[] }> };

function backendPath(path: string[]): string | null {
  if (
    path.length === 0 ||
    path.some(
      (segment) =>
        segment === "." ||
        segment === ".." ||
        !/^[A-Za-z0-9._~-]{1,128}$/.test(segment),
    )
  ) {
    return null;
  }
  return `/api/v1/${path.join("/")}`;
}

async function proxy(request: Request, context: RouteContext): Promise<Response> {
  const path = backendPath((await context.params).path);
  if (path === null) {
    return Response.json({ error: "invalid API path" }, { status: 400 });
  }
  const isJson = request.headers
    .get("content-type")
    ?.toLowerCase()
    .startsWith("application/json");
  return proxyBackendRequest(request, path, {
    ...(isJson ? { transformRequestBody: stripBrowserIdentityJsonRequestBody } : {}),
  });
}

export const GET = proxy;
export const HEAD = proxy;
export const POST = proxy;
export const PUT = proxy;
export const PATCH = proxy;
export const DELETE = proxy;
