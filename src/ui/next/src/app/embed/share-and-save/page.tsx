"use client";

import React, { useEffect, useState } from 'react';
import { useSearchParams } from 'next/navigation';

import { Suspense } from 'react';

function EmbedContent() {
    const searchParams = useSearchParams();
    const tenant = searchParams.get('tenant') || 'my-store';
    const theme = searchParams.get('theme') || 'light';
    const discount = searchParams.get('discount') || '10pct';
    const hideBranding = searchParams.get('hideBranding') === 'true';

    const [isMounted, setIsMounted] = useState(false);

    useEffect(() => {
        setIsMounted(true);
    }, []);

    if (!isMounted) return null;

    const discountAmount = discount.replace(/pct|flat/g, '');
    const discountType = discount.includes('flat') ? '$' : '%';

    return (
        <div className={`w-full min-h-full font-inter ${theme === 'dark' ? 'bg-gray-900 text-white' : 'bg-transparent text-gray-900'} flex flex-col justify-between`}>
            <div className="p-4 sm:p-6 text-center flex-1 flex flex-col justify-center">
                <div className={`w-10 h-10 sm:w-12 sm:h-12 rounded-full mx-auto mb-3 sm:mb-4 flex items-center justify-center ${theme === 'dark' ? 'bg-indigo-900/50 text-indigo-400' : 'bg-indigo-100 text-indigo-600'}`}>
                    <svg className="w-5 h-5 sm:w-6 sm:h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
                    </svg>
                </div>
                <h4 className="text-base sm:text-lg font-bold mb-1 sm:mb-2">Love our store?</h4>
                <p className={`text-xs sm:text-sm mb-4 sm:mb-5 leading-relaxed ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                    Share us with your friends on social media and get <strong className={theme === 'dark' ? 'text-white' : 'text-gray-900'}>{discountType === '$' ? '$' : ''}{discountAmount}{discountType === '%' ? '%' : ''} off</strong> your next order!
                </p>

                <div className="grid grid-cols-2 gap-2 sm:gap-3">
                    <button className="flex items-center justify-center gap-1.5 sm:gap-2 py-1.5 sm:py-2 px-3 sm:px-4 rounded-lg sm:rounded-xl bg-blue-600 text-white text-xs sm:text-sm font-medium hover:bg-blue-700 transition-colors shadow-sm">
                        <svg className="w-3.5 h-3.5 sm:w-4 sm:h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.469h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.469h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/></svg>
                        Share
                    </button>
                    <button className="flex items-center justify-center gap-1.5 sm:gap-2 py-1.5 sm:py-2 px-3 sm:px-4 rounded-lg sm:rounded-xl bg-black text-white text-xs sm:text-sm font-medium hover:bg-gray-800 transition-colors shadow-sm">
                        <svg className="w-3.5 h-3.5 sm:w-4 sm:h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                        Post
                    </button>
                </div>
            </div>

            {/* Powered By Watermark */}
            {!hideBranding && (
                <div className={`py-2 text-center text-[11px] sm:text-xs font-medium border-t ${theme === 'dark' ? 'border-gray-800 bg-gray-900/80' : 'border-gray-100 bg-white/80'} backdrop-blur-sm mt-auto`}>
                    <a
                        href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className={`hover:underline transition-colors ${theme === 'dark' ? 'text-gray-400 hover:text-gray-200' : 'text-gray-500 hover:text-gray-800'}`}
                    >
                        ⚡ Powered by OHC
                    </a>
                </div>
            )}
        </div>
    );
}

export default function ShareAndSaveEmbedPage() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <EmbedContent />
    </Suspense>
  );
}
