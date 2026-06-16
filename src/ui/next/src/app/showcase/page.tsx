"use client";

import React, { useState, useEffect, Suspense } from 'react';
import { PoweredByOHC } from '../components/PoweredByOHC';
import { useSearchParams } from 'next/navigation';

export default function PublicShowcasePage() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <ShowcaseContent />
    </Suspense>
  );
}

function ShowcaseContent() {
  const searchParams = useSearchParams();
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  if (!isClient) return null;

  const projectName = searchParams.get('p') || 'Project Showcase';
  const customerName = searchParams.get('c') || '';
  const description = searchParams.get('d') || '';
  const beforeImage = searchParams.get('b') || '';
  const afterImage = searchParams.get('a') || '';
  const ctaLink = searchParams.get('l') || '';
  const removeBranding = searchParams.get('r') === '1';
  const tenant = searchParams.get('t') || 'ohc';

  return (
    <div className="flex flex-col min-h-screen bg-[#F5F5F7] text-[#1D1D1F] font-inter">
      <main className="flex-1 overflow-y-auto p-4 md:p-8 flex items-center justify-center">
        <div className="max-w-3xl w-full mx-auto bg-white rounded-3xl shadow-2xl overflow-hidden border border-[#E5E5EA] flex flex-col min-h-[60vh]">

          {/* Showcase Content */}
          <div className="p-8 md:p-12 flex-1">
            <h1 className="text-3xl md:text-5xl font-bold tracking-tight mb-4">{projectName}</h1>

            {customerName && (
              <p className="text-sm md:text-base text-[#86868B] font-medium uppercase tracking-wider mb-8">For {customerName}</p>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
              {beforeImage && (
                <div className="space-y-3">
                  <div className="text-sm font-semibold text-[#86868B] tracking-widest uppercase">Before</div>
                  <div className="aspect-[4/3] bg-[#F5F5F7] rounded-2xl overflow-hidden flex items-center justify-center border border-[#E5E5EA] shadow-inner">
                    <img src={beforeImage} alt="Before" className="w-full h-full object-cover hover:scale-105 transition-transform duration-500" />
                  </div>
                </div>
              )}
              {afterImage && (
                <div className="space-y-3">
                  <div className="text-sm font-semibold text-[#86868B] tracking-widest uppercase">After</div>
                  <div className="aspect-[4/3] bg-[#F5F5F7] rounded-2xl overflow-hidden flex items-center justify-center border border-[#E5E5EA] shadow-inner">
                    <img src={afterImage} alt="After" className="w-full h-full object-cover hover:scale-105 transition-transform duration-500" />
                  </div>
                </div>
              )}
            </div>

            {description && (
              <div className="prose prose-lg max-w-none text-[#1D1D1F] mb-12">
                <p className="whitespace-pre-wrap leading-relaxed">{description}</p>
              </div>
            )}

            {ctaLink && (
              <div className="mt-10 pt-10 border-t border-[#E5E5EA] flex justify-center">
                <a
                  href={ctaLink}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="block w-full md:w-auto min-w-[280px] py-4 px-8 bg-[#1D1D1F] text-white text-center rounded-2xl font-bold text-lg hover:bg-black hover:scale-[1.02] transition-all shadow-lg hover:shadow-xl"
                >
                  Book a Similar Project
                </a>
              </div>
            )}
          </div>

          {/* Powered By OHC Loop */}
          {!removeBranding && (
            <div className="bg-[#F5F5F7] py-8 flex justify-center border-t border-[#E5E5EA]">
              <PoweredByOHC tenantId={tenant} />
            </div>
          )}
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
      `}} />
    </div>
  );
}
