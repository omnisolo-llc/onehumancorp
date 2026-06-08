import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    let backendRes;
    try {
        backendRes = await fetch(`${backendUrl}/api/v1/growth/promoter/generate`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(body),
        });
    } catch (e) {
        // Backend is down, fallback
    }

    if (backendRes && backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        const { product_name, description, theme, tenant } = body;
        const name = product_name || 'our latest product';
        const desc = description || 'Check it out now!';
        const storeLink = tenant ? `/bio/${tenant}` : '/store';

        let themeText = '';
        if (theme) {
            themeText = ` We're running a ${theme} special!`;
        }

        const instagram = `✨ Introducing ${name}! ✨\n\n${desc}${themeText}\n\nShop the collection at the link in our bio! 🛍️\n\n⚡ Powered by OHC`;

        const twitter = `🚨 NEW ARRIVAL 🚨\n\nGet your hands on ${name}. ${desc}${themeText}\n\nShop now: ${storeLink}\n\n⚡ Powered by OHC`;

        const email = `Subject: You're going to love our new ${name}! 🎉\n\nHi there,\n\nWe're thrilled to introduce our newest addition: ${name}.\n\n${desc}${themeText}\n\nWe think it's exactly what you've been looking for. Click below to shop before it sells out.\n\nShop now: ${storeLink}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;

        return NextResponse.json({
            instagram,
            twitter,
            email
        });
    }
  } catch (error) {
    console.error("Error generating promoter posts:", error);
    return NextResponse.json({ error: "Internal server error" }, { status: 500 });
  }
}
