import { NextResponse } from 'next/server';

export async function GET(req: Request, { params }: { params: { id: string } }) {
    try {
        const tenantId = req.headers.get("x-tenant-id") || "default";
        const apiUrl = process.env.API_URL || "http://127.0.0.1:8080";

        const res = await fetch(`${apiUrl}/v1/proposals/${params.id}`, {
            headers: {
                "x-tenant-id": tenantId,
            }
        });

        if (!res.ok) {
            return NextResponse.json({ success: false, error: "Not found" }, { status: res.status });
        }

        const data = await res.json();
        return NextResponse.json({ success: true, proposal: data.proposal });
    } catch(e) {
        return NextResponse.json({ success: false, error: "Network error" }, { status: 500 });
    }
}
