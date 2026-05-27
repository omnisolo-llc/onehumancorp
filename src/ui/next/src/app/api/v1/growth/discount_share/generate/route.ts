import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const tenantId = 'acme-corp'; // Mocked tenant ID for now
    // UUID v4 format mock
    const uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
      const r = Math.random() * 16 | 0;
      const v = c === 'x' ? r : (r & 0x3 | 0x8);
      return v.toString(16);
    });

    const shareUrl = `https://ohc.store/discount/${uuid}?tenant=${tenantId}`;

    return NextResponse.json({ share_url: shareUrl });
  } catch (error) {
    console.error("Error generating discount link:", error);
    return NextResponse.json(
      { error: "Failed to generate discount link" },
      { status: 500 }
    );
  }
}
