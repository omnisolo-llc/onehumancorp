import { NextResponse } from "next/server";

export function backendHeaders(req: Request, withJson = false): Headers {
  const headers = new Headers();
  if (withJson) headers.set("Content-Type", "application/json");

  for (const name of ["authorization", "cookie", "x-tenant-id", "x-user-id", "x-spiffe-id"]) {
    const value = req.headers.get(name);
    if (value) headers.set(name, value);
  }

  return headers;
}

export async function proxyBackendGet(req: Request, backendPath: string) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const { search } = new URL(req.url);

  try {
    const res = await fetch(`${backendUrl}${backendPath}${search}`, {
      method: "GET",
      headers: backendHeaders(req),
    });
    return NextResponse.json(await res.json(), { status: res.status });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
