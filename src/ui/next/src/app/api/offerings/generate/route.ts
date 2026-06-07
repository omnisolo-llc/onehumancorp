import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  let body: unknown;

  try {
    body = await req.json();
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }

  const intent = body && typeof body === 'object' && 'intent' in body
    ? (body as { intent?: string }).intent
    : undefined;

  if (typeof intent !== 'string' || !intent.trim()) {
    return NextResponse.json({ error: "Intent is required" }, { status: 400 });
  }

  try {
    // We leverage the existing auto-catalog or an agent endpoint.
    // Since we need structured data for a product offering based on text intent,
    // we use a specialized agent prompt routed through the backend agent framework.
    const res = await fetch(`${backendUrl}/api/agents/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        message: `Create a product/service offering from this intent: "${intent.trim()}". Return ONLY a JSON object with properties: title (string), description (string), price (string formatting number, e.g. "50.00"), category (string), type (either "Product" or "Service").`
      })
    });

    if (!res.ok) {
      throw new Error(`Backend AI responded with status: ${res.status}`);
    }

    const data = await res.json();
    let reply = data.reply || "";

    // The agent might wrap the JSON in markdown code blocks.
    reply = reply.replace(/```json/g, "").replace(/```/g, "").trim();

    let productData;
    try {
        productData = JSON.parse(reply);
    } catch (e) {
        // Fallback parsing if the LLM didn't return strict JSON
        console.warn("Failed to parse agent reply as JSON:", reply);
        productData = {
            title: intent.split(',')[0].substring(0, 30) || "New Offering",
            description: reply || "Auto-generated description based on intent.",
            price: "50.00",
            category: "General",
            type: intent.toLowerCase().includes('service') || intent.toLowerCase().includes('lesson') ? 'Service' : 'Product'
        };
    }

    return NextResponse.json(productData);

  } catch (error) {
    console.error("Failed to connect to backend AI agent for generation:", error);
    return NextResponse.json({
      error: "Failed to generate offering from AI."
    }, { status: 500 });
  }
}
