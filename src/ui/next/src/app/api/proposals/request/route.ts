import { NextResponse } from 'next/server';

export async function POST(req: Request) {
    try {
        const body = await req.json();

        const tenantId = req.headers.get("x-tenant-id") || "default";
        const apiUrl = process.env.API_URL || "http://127.0.0.1:8080";

        const res = await fetch(`${apiUrl}/v1/proposals/request`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "x-tenant-id": tenantId,
            },
            body: JSON.stringify({
                tenantId: tenantId,
                description: body.description,
                customerName: body.customer_name,
                customerEmail: body.customer_email,
            })
        });

        if (!res.ok) {
            return NextResponse.json({ success: false, error: "Backend failed" }, { status: res.status });
        }

        const data = await res.json();
        return NextResponse.json({ success: true, inquiry_id: data.inquiryId });
    } catch(e) {
        return NextResponse.json({ success: false, error: "Network error" }, { status: 500 });
    }
}
