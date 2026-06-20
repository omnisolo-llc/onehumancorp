import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    try {
        const formData = await request.formData();

        // Pass payload to backend
        const bodyData = new URLSearchParams();
        for (const [key, value] of formData.entries()) {
            bodyData.append(key, value.toString());
        }

        const response = await fetch(`${process.env.OHC_API_URL || 'http://localhost:18789'}/api/agents/client_intake?tenant=default`, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            },
            method: 'POST',
            body: bodyData.toString(),
        });

        const data = await response.json();
        return NextResponse.json(data, { status: response.status });
    } catch (e: any) {
        console.error("Error processing client intake:", e);
        return NextResponse.json({ success: false, proposal_drafted: false }, { status: 500 });
    }
}
