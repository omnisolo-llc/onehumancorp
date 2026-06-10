"use client";

import React, { useEffect, useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function ShareAndSaveEmbedContent() {
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
                <h3 className="text-lg sm:text-xl font-bold mb-1 sm:mb-2">Get {discountType === '$' ? '$' : ''}{discountAmount}{discountType === '%' ? '%' : ''} Off</h3>
                <p className={`text-xs sm:text-sm mb-4 sm:mb-6 ${theme === 'dark' ? 'text-gray-300' : 'text-gray-600'}`}>
                    Share your unique link with a friend. They get a discount, and you get a reward when they buy!
                </p>
                <div className="flex items-center gap-2 max-w-sm mx-auto w-full">
                    <input
                        type="text"
                        readOnly
                        value={`https://${tenant}.ohc.app/?ref=USER123`}
                        className={`flex-1 p-2 sm:p-3 text-xs sm:text-sm rounded-lg border outline-none ${theme === 'dark' ? 'bg-gray-800 border-gray-700 text-gray-300' : 'bg-white border-gray-200 text-gray-600'}`}
                    />
                    <button className={`px-3 py-2 sm:px-4 sm:py-3 text-xs sm:text-sm font-semibold rounded-lg shrink-0 ${theme === 'dark' ? 'bg-indigo-500 hover:bg-indigo-400 text-white' : 'bg-indigo-600 hover:bg-indigo-700 text-white'}`}>
                        Copy Link
                    </button>
                </div>
            </div>

            {!hideBranding && (
                <div className={`p-2 text-center text-[10px] font-medium tracking-wide border-t ${theme === 'dark' ? 'border-gray-800 text-gray-500' : 'border-gray-100 text-gray-400'}`}>
                    POWERED BY OHC GROWTH
                </div>
            )}
        </div>
    );
}

export default function ShareAndSaveEmbedPage() {
  return (
    <Suspense>
      <ShareAndSaveEmbedContent />
    </Suspense>
  )
}
