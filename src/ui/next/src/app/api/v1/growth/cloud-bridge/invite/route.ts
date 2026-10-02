import { NextRequest, NextResponse } from "next/server";
import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

export async function POST(request: Request | NextRequest) {
  try {
    // The authenticated backend supplies team and inviter authority. Preserve
    // its actual status, receipt URL and error body without manufacturing links.
    return await proxyBackendRequest(request, "/api/v1/growth/team-invites", {
      requestContentType: "application/json",
      transformRequestBody(body) {
        const payload = body.byteLength === 0 ? {} : JSON.parse(decoder.decode(body));
        return encoder.encode(JSON.stringify({ invitee_id: payload.invitee_id ?? "" }));
      },
    });
  } catch {
    return NextResponse.json(
      { error: "Invite service temporarily unavailable" },
      { status: 503 }
    );
  }
}

export async function GET(request: Request | NextRequest) {
  try {
    return await proxyBackendRequest(request, "/api/v1/growth/team-invites", {
      suppressRequestBody: true,
    });
  } catch {
    return NextResponse.json(
      { error: "Invite service temporarily unavailable" },
      { status: 503 }
    );
  }
}
