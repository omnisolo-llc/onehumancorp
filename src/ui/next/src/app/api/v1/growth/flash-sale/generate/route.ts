import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    try {
      const backendRes = await fetch(`${backendUrl}/api/v1/growth/flash-sale/generate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });

      if (backendRes.ok) {
          const data = await backendRes.json();
          return NextResponse.json(data);
      }
    } catch (fetchError) {
      console.log('Falling back to local generation for flash sale.');
    }

    const { product, discount, duration, tenant } = body;
    const store = tenant || 'our store';
    const prod = product || 'everything';
    const disc = discount || '20';
    const dur = duration || '24';

    const draft = `🚨 FLASH SALE ALERT! 🚨\n\nFor the next ${dur} hours ONLY, we're giving you ${disc}% OFF ${prod}!\n\nDon't miss out on these incredible savings at ${store}. Once the clock runs out, the prices go back up.\n\nShop the sale now: https://${store.toLowerCase().replace(/[^a-z0-9]/g, '')}.ohc.store\n\nHurry! ⏰\n\n⚡ Powered by OHC`;

    const endTime = new Date(Date.now() + parseInt(dur) * 60 * 60 * 1000).toISOString();

    const snippet = `<!-- Flash Sale Countdown Banner -->\n<div id="ohc-flash-sale" data-product="${prod}" data-discount="${disc}" data-end="${endTime}"></div>\n<script src="https://ohc.app/widgets/flash-sale.js" async></script>\n<!-- ⚡ Powered by OHC -->`;
