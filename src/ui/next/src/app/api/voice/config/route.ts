import { NextRequest, NextResponse } from 'next/server';

const API_BASE_URL = process.env.API_BASE_URL || 'http://127.0.0.1:18789';

export async function GET(req: NextRequest) {
    try {
        const token = req.headers.get('Authorization');
        const response = await fetch(`${API_BASE_URL}/api/v1/voice/config`, {
            method: 'GET',
            headers: {
                'Authorization': token || '',
            },
        });

        if (!response.ok) {
            console.error(`Backend returned ${response.status} for voice config`);
            return NextResponse.json({
                phone_number: "(555) 123-4567",
                is_enabled: false,
                primary_language: "English",
                custom_instructions: ""
            });
        }

        const data = await response.json();
        return NextResponse.json(data);
    } catch (e) {
        console.error("Failed to fetch voice config", e);
        return NextResponse.json({
            phone_number: "(555) 123-4567",
            is_enabled: false,
            primary_language: "English",
            custom_instructions: ""
        });
    }
}

export async function POST(req: NextRequest) {
    try {
        const token = req.headers.get('Authorization');
        const body = await req.json();

        const response = await fetch(`${API_BASE_URL}/api/v1/voice/config`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': token || '',
            },
            body: JSON.stringify(body),
        });

        if (!response.ok) {
            console.error(`Backend returned ${response.status} for POST voice config`);
            return NextResponse.json({ error: 'Failed to update config' }, { status: 500 });
        }

        const data = await response.json();
        return NextResponse.json(data);
    } catch (e) {
        console.error("Failed to update voice config", e);
        return NextResponse.json({ error: 'Failed to update config' }, { status: 500 });
    }
}
