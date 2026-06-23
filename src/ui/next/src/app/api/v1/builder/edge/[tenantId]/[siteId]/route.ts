import { NextRequest, NextResponse } from "next/server";

export async function GET(req: NextRequest, context: { params: Promise<{ tenantId: string; siteId: string }> }) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const p = await context.params;
  const { tenantId, siteId } = p;

  try {
    const res = await fetch(`${backendUrl}/api/v1/builder/edge/${tenantId}/${siteId}`, {
      next: { revalidate: 60 }
    });
    if (!res.ok) {
       return new NextResponse(await res.text(), { status: res.status });
    }

    const headers = new Headers();
    headers.set('Cache-Control', 'public, s-maxage=60, stale-while-revalidate=86400');
    headers.set('Content-Type', 'text/html');

    const cacheTag = res.headers.get('Cache-Tag');
    if (cacheTag) {
        headers.set('Cache-Tag', cacheTag);
    }

    return new NextResponse(await res.text(), { headers, status: 200 });
  } catch (error) {
    return new NextResponse("Internal Server Error", { status: 500 });
  }
}
