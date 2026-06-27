import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { restaurantName, description, menuItemsText, tenantId } = await request.json();

    // Use environment variable for backend URL, default to a sensible local value for testing
    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';

    const escapeHtml = (unsafe: string) => {
        if (!unsafe) return unsafe;
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    };

    const safeRestaurantName = escapeHtml(restaurantName) || 'Our Menu';
    const safeDescription = escapeHtml(description) || '';
    const safeMenuItemsText = escapeHtml(menuItemsText) || '';
    const safeTenantId = escapeHtml(tenantId) || 'demo';

    // In a real implementation this would use the configured LLM to generate the menu and persist it
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-menu`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        restaurant_name: safeRestaurantName,
        description: safeDescription,
        items_text: safeMenuItemsText,
        tenant_id: safeTenantId,
      })
    });

    if (!backendRes.ok) { return NextResponse.json({ error: "Backend error" }, { status: 502 }); }

    const data = await backendRes.json();
    return NextResponse.json(data);

  } catch (error) { return NextResponse.json({ error: "Network error" }, { status: 502 }); }
}
