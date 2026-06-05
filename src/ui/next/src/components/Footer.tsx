import React from 'react';

interface FooterProps {
  tenantId?: string;
  theme?: 'light' | 'dark' | 'gradient';
  className?: string;
}

export function Footer({ tenantId, theme = 'light', className = '' }: FooterProps) {
  // Determine referral link based on tenantId if available
  const referralLink = tenantId
    ? `https://ohc.store/join?ref=${encodeURIComponent(tenantId)}`
    : 'https://ohc.store/join';

  // Determine styling based on theme
  let textColorClass = 'text-gray-500';
  let hoverColorClass = 'hover:text-gray-800';

  if (theme === 'dark') {
    textColorClass = 'text-gray-400';
    hoverColorClass = 'hover:text-white';
  } else if (theme === 'gradient') {
    textColorClass = 'text-white/80';
    hoverColorClass = 'hover:text-white';
  }

  return (
    <div className={`mt-8 mb-4 w-full flex justify-center items-center ${className}`}>
      <a
        href={referralLink}
        target="_blank"
        rel="noopener noreferrer"
        className={`flex items-center gap-1.5 text-xs sm:text-sm font-semibold tracking-wider uppercase transition-all duration-300 ${textColorClass} ${hoverColorClass}`}
        style={{ textDecoration: 'none' }}
      >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            className="w-4 h-4"
        >
          <path fillRule="evenodd" d="M14.615 1.595a.75.75 0 01.359.852L12.982 9.75h7.268a.75.75 0 01.548 1.262l-10.5 11.25a.75.75 0 01-1.272-.71l1.992-7.302H3.75a.75.75 0 01-.548-1.262l10.5-11.25a.75.75 0 01.913-.143z" clipRule="evenodd" />
        </svg>
        Powered by OHC
      </a>
    </div>
  );
}
