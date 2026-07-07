import { NextResponse, NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";

  try {
    const res = await fetch(`${backendUrl}/api/changelog`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      if (data && data.length > 0) {
        return NextResponse.json(data);
      }
    }

    return NextResponse.json([{ version: "v1.0.0", contentLines: ["### New Features", "- Initial release"] }]);
  } catch (e) {
    return NextResponse.json([{ version: "v1.0.0", contentLines: ["### New Features", "- Initial release"] }]);
  }
}
