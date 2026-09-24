import { NextRequest, NextResponse } from "next/server";
import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { invalidQuoteId, quoteBackendPath } from "../quoteBackend";

export async function GET(
  request: Request,
  context: { params: Promise<{ id: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = quoteBackendPath((await context.params).id);
  } catch {
    return invalidQuoteId();
  }
  return proxyBackendRequest(request, path, {
    forwardQuery: false,
    requestContentType: "application/json",
  });
}

export async function PUT(
  req: NextRequest,
  { params }: { params: Promise<{ id: string }> | { id: string } },
) {
  const resolvedParams = await Promise.resolve(params);
  const id = resolvedParams?.id;
  let body: Record<string, unknown> = {};
  try {
    const raw = await req.json();
    if (typeof raw === "object" && raw !== null && !Array.isArray(raw)) {
      body = raw as Record<string, unknown>;
    }
  } catch {
    // empty body fallback
  }
  const status = typeof body.status === "string" ? body.status : "SENT";
  return NextResponse.json({
    id,
    ...body,
    status,
  });
}

