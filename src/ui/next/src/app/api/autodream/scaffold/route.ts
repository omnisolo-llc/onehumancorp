import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { prompt } = await request.json();

    if (!prompt) {
      return NextResponse.json({ error: 'Prompt is required' }, { status: 400 });
    }

    // In a full implementation, this would call the Rust backend's `AutodreamPipeline` service
    // For now, we simulate the backend processing based on the user's prompt.
    // We are generating a complete business scaffold.

    // Simple heuristic for simulation
    const lowerPrompt = prompt.toLowerCase();
    let businessType = "Service Business";
    let businessName = "My New Business";
    let products = [];

    if (lowerPrompt.includes("cake") || lowerPrompt.includes("bake")) {
      businessType = "Home Bakery";
      businessName = "Maya's Cakes";
      products = [
        { name: "Custom Vegan Chocolate Cake", price: "45.00", category: "food" },
        { name: "Dozen Vanilla Cupcakes", price: "24.00", category: "food" }
      ];
    } else if (lowerPrompt.includes("plumb") || lowerPrompt.includes("repair") || lowerPrompt.includes("handyman")) {
      businessType = "Handyman Services";
      businessName = "Carlos Repairs";
      products = [
        { name: "General Hourly Repair", price: "75.00", category: "services" },
        { name: "Emergency Plumbing Fix", price: "150.00", category: "services" }
      ];
    } else if (lowerPrompt.includes("tutor") || lowerPrompt.includes("teach") || lowerPrompt.includes("guitar")) {
      businessType = "Tutoring";
      businessName = "Leo's Guitar Lessons";
      products = [
        { name: "1 Hour Guitar Lesson", price: "50.00", category: "services" },
        { name: "4-Lesson Monthly Package", price: "180.00", category: "services" }
      ];
    } else {
      businessType = "Online Store";
      products = [
        { name: "Standard Product", price: "19.99", category: "physical" },
        { name: "Premium Service", price: "99.99", category: "services" }
      ];
    }

    const tenantId = `org-${Math.random().toString(36).substring(2, 9)}`;

    // Simulate backend processing time for scaffolding (DB creation, agent seeding, etc.)
    await new Promise(resolve => setTimeout(resolve, 1500));

    return NextResponse.json({
      success: true,
      tenant_id: tenantId,
      business_name: businessName,
      business_type: businessType,
      products: products,
      message: `Successfully scaffolded ${businessName} as a ${businessType}`
    });

  } catch (e: any) {
    console.error(`Scaffold API error: ${e.message}`);
    return NextResponse.json({ error: 'Failed to process scaffolding request' }, { status: 500 });
  }
}
