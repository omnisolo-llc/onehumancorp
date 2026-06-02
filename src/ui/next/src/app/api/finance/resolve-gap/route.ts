import { NextResponse } from 'next/server';

export async function POST(req: Request) {
    const data = await req.json();

    // Mock resolution
    if (data.action === "send_reminders") {
        return NextResponse.json({ success: true, message: "Invoice reminders sent successfully. Estimated gap covered: $400." });
    } else if (data.action === "take_advance") {
        return NextResponse.json({ success: true, message: "Advance of $500 approved. Funds are available instantly." });
    }

    return NextResponse.json({ success: false, error: "Invalid action" }, { status: 400 });
}
