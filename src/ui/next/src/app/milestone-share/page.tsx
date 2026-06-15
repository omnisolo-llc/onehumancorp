import { Metadata, ResolvingMetadata } from 'next';
import React from 'react';

type Props = {
  searchParams: { [key: string]: string | string[] | undefined }
}

const defaultTargetUrl = '/onboarding';

export async function generateMetadata(
  { searchParams }: Props,
  _parent: ResolvingMetadata
): Promise<Metadata> {
  const milestoneId = typeof searchParams.milestone === 'string' ? searchParams.milestone : 'first_sale';
  const tenantId = typeof searchParams.tenant === 'string' ? searchParams.tenant : 'DEFAULT';

  const title = typeof searchParams.title === 'string' ? searchParams.title : 'Milestone Reached! 🏆';
  const description = typeof searchParams.description === 'string' ? searchParams.description : 'We just hit a massive business milestone on One Human Corp! Start your own business today.';
  const imageUrl = `/api/v1/growth/milestone/card?milestone_id=${milestoneId}&tenant=${tenantId}`;

  // Hardcode absolute origin for API if possible, otherwise rely on relative, but OG tags typically need absolute URLs.
  // Next.js will resolve absolute URLs if metadataBase is set, but we can also manually prepend a dummy if needed, though typically OHC provides absolute where possible.
  // We'll leave it relative; if Next.js needs absolute, we'll assume it's handled or we'll pass full URL from frontend. Let's construct a full URL.
  // For safety, we'll try to extract host from headers in a real app, but here we can just pass the path and let NextJS handle it if metadataBase is configured, or we require an absolute URL.
  // In OHC, we'll use a placeholder domain for the test or just relative.
  const absoluteImageUrl = `https://ohc.app${imageUrl}`; // Fallback for OG

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      images: [absoluteImageUrl],
      type: 'website',
    },
    twitter: {
      card: 'summary_large_image',
      title,
      description,
      images: [absoluteImageUrl],
    },
  };
}

export default function MilestoneSharePage({ searchParams }: Props) {
  const milestoneId = typeof searchParams.milestone === 'string' ? searchParams.milestone : 'first_sale';
  const tenantId = typeof searchParams.tenant === 'string' ? searchParams.tenant : 'DEFAULT';

  const cardUrl = `/api/v1/growth/milestone/card?milestone_id=${milestoneId}&tenant=${tenantId}`;
  const joinUrl = `/onboarding?ref=${tenantId}`;

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center justify-center p-6 font-sans">
      <div className="max-w-3xl w-full bg-white rounded-3xl shadow-2xl overflow-hidden border border-gray-100">
        <div className="w-full aspect-[1200/630] bg-gray-100 relative">
            <img
              src={cardUrl}
              alt="Milestone"
              className="w-full h-full object-cover"
              onError={(e) => { e.currentTarget.src = 'data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMjAwIiBoZWlnaHQ9IjYzMCI+PHJlY3Qgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgZmlsbD0iI2YxZjVmOSIvPjx0ZXh0IHg9IjUwJSIgeT0iNTAlIiBmb250LWZhbWlseT0ic2Fucy1zZXJpZiIgZm9udC1zaXplPSI0OCIgZmlsbD0iIzk0YTNiOSIgdGV4dC1hbmNob3I9Im1pZGRsZSI+Q291bGQgbm90IGxvYWQgbWlsZXN0b25lIGNhcmQ8L3RleHQ+PC9zdmc+'; }}
            />
        </div>

        <div className="p-8 md:p-12 text-center flex flex-col items-center">
            <h1 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4 tracking-tight">Celebrate this milestone!</h1>
            <p className="text-lg text-gray-600 mb-8 max-w-xl">
              This business is powered by One Human Corp. Want to achieve your own milestones? Launch your business online in seconds.
            </p>
            <a
              href={joinUrl}
              className="inline-flex items-center justify-center px-8 py-4 text-base font-bold text-white bg-indigo-600 hover:bg-indigo-700 rounded-full transition-all shadow-lg hover:shadow-xl hover:-translate-y-0.5"
            >
              Start your own business on OHC
              <svg className="w-5 h-5 ml-2 -mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7l5 5m0 0l-5 5m5-5H6" /></svg>
            </a>
            <p className="mt-4 text-sm text-gray-500 font-medium">Get a $50 credit when you join today.</p>
        </div>
      </div>
    </div>
  );
}
