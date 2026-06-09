"use client";
import React, { Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function EmbedContent() {
  const searchParams = useSearchParams();
  const tenant = searchParams.get('tenant') || 'default-tenant';
  const theme = searchParams.get('theme') || 'light';
  const title = searchParams.get('title') || 'Special Offer';
  const description = searchParams.get('desc') || 'Claim your offer now.';
  const buttonText = searchParams.get('btn') || 'Get Offer';
  const buttonLink = searchParams.get('url') || '#';

  const isDark = theme === 'dark';

  // Note: For removeBranding, if we wanted to enforce it securely, we'd check the server backend.
  // For the widget level, we assume if it's not present, we show the branding.
  const branding = searchParams.get('branding') !== 'false';

  return (
    <div
      className={`min-h-screen flex flex-col items-center justify-center text-center p-6 transition-colors ${
        isDark ? 'bg-gray-900 text-white' : 'bg-white text-gray-900'
      }`}
    >
      <h3 className="text-xl font-bold font-outfit mb-2">{title}</h3>
      <p className={`text-sm mb-6 ${isDark ? 'text-gray-300' : 'text-gray-600'}`}>{description}</p>

      <a
        href={buttonLink}
        target="_blank"
        rel="noopener noreferrer"
        className="inline-block px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors"
      >
        {buttonText}
      </a>

      {branding && (
        <div className="mt-4 pt-4 border-t border-gray-200/20 w-full opacity-60 hover:opacity-100 transition-opacity">
          <a
            href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}`}
            target="_blank"
            rel="noreferrer"
            className={`text-xs font-semibold flex items-center justify-center gap-1 ${
              isDark ? 'text-gray-400 hover:text-white' : 'text-gray-500 hover:text-gray-900'
            }`}
          >
            ⚡ Powered by OHC
          </a>
        </div>
      )}
    </div>
  );
}

export default function EmbedOfferPage() {
  return (
    <Suspense fallback={<div className="p-4 text-center">Loading...</div>}>
      <EmbedContent />
    </Suspense>
  );
}
