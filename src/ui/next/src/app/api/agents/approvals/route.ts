import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

import { headers } from "next/headers";

export async function GET(request: Request) {
  // Use absolute URL to bypass Next.js and hit the Rust API proxy if running in a hybrid setup,
  // or proxy directly.
  try {
    const backendUrl = process.env.BACKEND_API_URL || 'http://127.0.0.1:8080';
    const res = await fetch(`${backendUrl}/api/agents/approvals`, {
        headers: headers(),
    });
    if (!res.ok) {
        return NextResponse.json({ pending_approvals: [] });
    }
    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json({ pending_approvals: [] });
  }
}
