// Provide a mocked or updated route since pg is uninstalled
import { NextResponse } from 'next/server';

export async function POST() {
    return NextResponse.json({ success: true });
}
