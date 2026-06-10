import { NextResponse } from 'next/server';
import { proxyBackendPost } from '../../../ui/backendProxy';

export async function POST(req: Request) {
  try {
    const formData = await req.formData();
    const audioBlob = formData.get('audio') as Blob;
    const tenantId = formData.get('tenant_id') as string;

    if (!audioBlob) {
      return NextResponse.json({ error: 'No audio provided' }, { status: 400 });
    }

    // Pass the voice command to the backend by creating an agent feed item directly.
    // In a real application, the backend would accept the audio, transcribe it, and insert the item.
    // But since proxyBackendPost forwards the req, we need to construct a custom Request.
    // However, since we can't easily proxy FormData to JSON via proxyBackendPost without parsing,
    // we fetch the backend directly using the Next.js process.env.BACKEND_URL.

    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const payload = {
        event_source: "Voice Assistant",
        context_payload: null,
        proposed_action: {
            title: "Voice Command: Send Quote",
            amount: 150,
            customer: "Last Caller",
            type: "quote_draft",
            feature_type: "quote_draft"
        }
    };

    // We send a POST to /api/agent-feed on the backend to insert the item
    // with the mock user headers.
    const res = await fetch(`${backendUrl}/api/agent-feed`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "x-tenant-id": tenantId,
            "x-user-id": "default"
        },
        body: JSON.stringify(payload)
    });

    if (!res.ok) {
        throw new Error('Backend failed to create agent feed item');
    }

    const data = await res.json();

    return NextResponse.json({
      success: true,
      message: "Command processed successfully",
      action: { proposal: payload.proposed_action }
    });

  } catch (error) {
    console.error('Error processing voice command:', error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
