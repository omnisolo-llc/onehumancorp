import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const formData = await request.formData();
    const title = formData.get('title')?.toString() || '';
    const imageFile = formData.get('image') as File | null;
    let image = null;
    if (imageFile) {
        const buffer = await imageFile.arrayBuffer();
        image = Buffer.from(buffer).toString('base64');
    }

    // Call the backend endpoint. The backend handles Minimax LLM logic and returns the generated content.
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    // Get cookie/auth context if possible
    const cookieStr = request.headers.get('cookie') || '';
    const tenantId = request.headers.get('x-tenant-id') || 'org1';
    const userId = request.headers.get('x-user-id') || 'user1';

    const res = await fetch(`${backendUrl}/api/v1/growth/copywriter/generate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId,
        'cookie': cookieStr
      },
      body: JSON.stringify({ title, image_data: image })
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to rewrite' }, { status: res.status });
  } catch (error) {
    console.error("Error generating text:", error);
    return NextResponse.json(
      { error: "Failed to generate text" },
      { status: 500 }
    );
  }
}
