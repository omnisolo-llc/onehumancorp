import React from 'react';

export const LivePreviewCard = ({
  theme,
  serviceName,
  onBook,
  previewStatus,
  removeBranding,
  tenant
}: {
  theme: 'light' | 'dark';
  serviceName: string;
  onBook: () => void;
  previewStatus: string;
  removeBranding: boolean;
  tenant: string;
}) => (
  <div className="relative z-10 w-[320px] h-[400px]" style={{
    background: theme === 'dark' ? '#1c1c1e' : '#ffffff',
    color: theme === 'dark' ? '#ffffff' : '#111827',
    borderRadius: '16px',
    boxShadow: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)'
  }}>
    <div className="w-full h-48 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-t-[16px] relative flex items-center justify-center">
        <span className="text-4xl text-white">📅</span>
        <div className="absolute top-3 right-3 bg-white/20 backdrop-blur-[30px] saturate-[210%] border border-white/30 text-white text-xs font-bold px-3 py-1 rounded-full">
            Book Now
        </div>
    </div>
    <div className="p-5 flex flex-col h-[208px]">
        <h4 className="font-bold text-lg font-outfit mb-1">{serviceName}</h4>
        <p className="text-sm mb-4 line-clamp-2" style={{ color: theme === 'dark' ? '#d1d5db' : '#4b5563' }}>
            Schedule your appointment with us easily. Tell us what you need and we will get right back to you.
        </p>
        <button
            type="button"
            onClick={onBook}
            className="w-full mt-auto py-2.5 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-lg text-sm flex items-center justify-center gap-2 transition-colors"
        >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
            Request a Service
        </button>
        {previewStatus && <p className="mt-2 text-xs font-semibold text-[#0071E3]" role="status">{previewStatus}</p>}
    </div>
  </div>
);
