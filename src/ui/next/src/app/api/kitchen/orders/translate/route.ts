import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const notes = body.notes || "";

    // Simulate the Operations Agent translating the notes.
    // In a real implementation this would call an LLM backend service.
    let translatedNotes = "";
    if (notes.toLowerCase().includes("no onions")) {
      translatedNotes = "بدون بصل";
    } else if (notes.toLowerCase().includes("extra pita")) {
      translatedNotes = "خبز إضافي";
    } else {
      translatedNotes = "ترجمة مبدئية: " + notes; // Default fallback simulation
    }

    return NextResponse.json({ translatedNotes });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to translate order notes' }, { status: 500 });
  }
}
