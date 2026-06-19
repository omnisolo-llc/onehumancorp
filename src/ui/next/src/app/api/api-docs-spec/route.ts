import { NextResponse, NextRequest } from 'next/server';

export const dynamic = 'force-dynamic';

const fallbackSpec = {
  "openapi": "3.0.0",
  "info": {
      "title": "OHC Advanced API Reference (Fallback)",
      "version": "1.0.0",
      "description": "API Reference for advanced users integrating with OneHumanCorp.",
  },
  "servers": [
      {
          "url": "http://localhost:8080",
      }
  ],
  "paths": {}
};

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const res = await fetch(`${backendUrl}/api/api-docs-spec`);

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json(fallbackSpec, { status: 200 });
  } catch (e) {
    if (process.env.NODE_ENV !== "test") console.error("Failed to fetch api-docs-spec from backend:", e);
    return NextResponse.json(fallbackSpec, { status: 200 });
  }
}
