import { NextResponse } from "next/server";

export async function POST(
  request: Request,
  { params }: { params: { id: string } }
) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const tenantId = request.headers.get("x-tenant-id") || "default";

  try {
    const res = await fetch(`${backendUrl}/quotes/${params.id}/approve`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
        "x-tenant-id": tenantId,
      },
    });

    if (res.ok) {
      // Typically, the backend should return a stripe checkout URL
      // Since it's not fully implemented on the rust side returning a real checkout URL,
      // we mock the checkout URL redirect here for demonstration.
      const mockCheckoutUrl = `/proposal/${params.id}?success=true`;

      return NextResponse.json({
        success: true,
        checkoutUrl: mockCheckoutUrl
      });
    }

    return NextResponse.json({ error: "Failed to approve proposal" }, { status: res.status });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
