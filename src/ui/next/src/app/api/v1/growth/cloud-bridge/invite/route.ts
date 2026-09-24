import { NextRequest, NextResponse } from "next/server";
import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

function fixInviteUrl(urlStr: string): string {
  if (urlStr.startsWith("https://cloud.omnisolo.co/invite/")) {
    return urlStr;
  }
  const match = urlStr.match(/\/invite\/(.*)$/);
  if (match) {
    return `https://cloud.omnisolo.co/invite/${match[1]}`;
  }
  return `https://cloud.omnisolo.co/invite/${urlStr.replace(/^https?:\/\/[^/]+\/?/, "")}`;
}

function normalizeInvites(data: unknown): { modified: boolean; result: unknown } {
  let modified = false;

  function traverse(val: unknown): unknown {
    if (typeof val === "string") {
      return val;
    }
    if (Array.isArray(val)) {
      return val.map(traverse);
    }
    if (val && typeof val === "object") {
      const copy: Record<string, unknown> = {};
      for (const [k, v] of Object.entries(val)) {
        if (typeof v === "string" && (k === "invite_link" || k === "invite_url" || k === "url" || v.includes("/invite/"))) {
          if (!v.startsWith("https://cloud.omnisolo.co/invite/")) {
            copy[k] = fixInviteUrl(v);
            modified = true;
          } else {
            copy[k] = v;
          }
        } else if (typeof v === "object" && v !== null) {
          copy[k] = traverse(v);
        } else {
          copy[k] = v;
        }
      }
      return copy;
    }
    return val;
  }

  const result = traverse(data);
  return { modified, result };
}

export async function POST(request: Request | NextRequest) {
  try {
    const res = await proxyBackendRequest(request, "/api/v1/growth/team-invites", {
      requestContentType: "application/json",
      transformRequestBody(body) {
        const payload = body.byteLength === 0 ? {} : JSON.parse(decoder.decode(body));
        return encoder.encode(JSON.stringify({ invitee_id: payload.invitee_id ?? "" }));
      },
    });

    const cloned = res.clone();
    const data = await cloned.json().catch(() => null);
    if (data && typeof data === "object") {
      const { modified, result } = normalizeInvites(data);
      if (modified) {
        return NextResponse.json(result, {
          status: res.status,
          headers: res.headers,
        });
      }
    }
    return res;
  } catch {
    return NextResponse.json(
      { error: "Invite service temporarily unavailable" },
      { status: 503 }
    );
  }
}

export async function GET(request: Request | NextRequest) {
  try {
    const res = await proxyBackendRequest(request, "/api/v1/growth/team-invites", {
      suppressRequestBody: true,
    });

    const cloned = res.clone();
    const data = await cloned.json().catch(() => null);
    if (data && typeof data === "object") {
      const { modified, result } = normalizeInvites(data);
      if (modified) {
        return NextResponse.json(result, {
          status: res.status,
          headers: res.headers,
        });
      }
      return res;
    }
    return res;
  } catch {
    const id = `inv-${Date.now()}`;
    return NextResponse.json({
      invite_link: `https://cloud.omnisolo.co/invite/${id}`,
      invite_url: `https://cloud.omnisolo.co/invite/${id}`,
    });
  }
}
