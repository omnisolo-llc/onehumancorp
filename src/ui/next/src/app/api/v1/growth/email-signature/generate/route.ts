import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const data = await request.json();
    const { name, role, company, phone, website, bannerText, tenantId, hasPro, removeBranding } = data;

    const escapeHtml = (unsafe: string) => {
        if (!unsafe) return unsafe;
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    };

    const safeName = escapeHtml(name) || 'Your Name';
    const safeRole = escapeHtml(role) || 'Your Role';
    const safeCompany = escapeHtml(company) || 'Your Company';
    const safePhone = escapeHtml(phone);
    const safeWebsite = escapeHtml(website);
    const safeBanner = escapeHtml(bannerText);

    let html = `<div style="font-family: Arial, sans-serif; font-size: 14px; color: #333; line-height: 1.5; padding: 10px 0; max-width: 400px;">`;
    html += `<div style="font-weight: bold; font-size: 16px; color: #111;">${safeName}</div>`;
    html += `<div style="color: #666;">${safeRole} | ${safeCompany}</div>`;

    if (safePhone || safeWebsite) {
      html += `<div style="margin-top: 8px; font-size: 13px;">`;
      if (safePhone) html += `<span style="color: #555;">📞 ${safePhone}</span>`;
      if (safePhone && safeWebsite) html += ` <span style="color: #ccc;">|</span> `;
      if (safeWebsite) html += `<a href="${safeWebsite.startsWith('http') ? safeWebsite : 'https://' + safeWebsite}" style="color: #4F46E5; text-decoration: none;">🌐 ${safeWebsite}</a>`;
      html += `</div>`;
    }

    if (safeBanner) {
      html += `<div style="margin-top: 12px; padding: 8px 12px; background-color: #EEF2FF; border-left: 3px solid #4F46E5; border-radius: 4px; font-size: 13px; font-weight: 600;">`;
      html += `<a href="${safeWebsite ? (safeWebsite.startsWith('http') ? safeWebsite : 'https://' + safeWebsite) : '#'}" style="color: #4F46E5; text-decoration: none;">${safeBanner} ✨</a>`;
      html += `</div>`;
    }

    // Branding viral loop
    if (!removeBranding) {
      const referralLink = `https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenantId || 'demo')}`;
      html += `<div style="margin-top: 16px; padding-top: 8px; border-top: 1px solid #eee; font-size: 11px; text-align: left;">`;
      html += `<a href="${referralLink}" target="_blank" style="color: #9CA3AF; text-decoration: none; font-weight: bold;">⚡ Powered by OHC</a>`;
      html += `</div>`;
    }

    html += `</div>`;

    return NextResponse.json({ html });
  } catch (error) {
    console.error("Error generating email signature:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
