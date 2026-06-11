"use client";

import React, { useState, useEffect, useRef } from 'react';

interface PoweredByOHCProps {
  tenantId: string;
}

export function PoweredByOHC({ tenantId }: PoweredByOHCProps) {
  const referralUrl = `/onboarding?ref=${tenantId}&source=footer_widget`;
  const [isHovered, setIsHovered] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  // Close popover when clicking outside on mobile
  useEffect(() => {
    function handleClickOutside(event: MouseEvent | TouchEvent) {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsHovered(false);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("touchstart", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("touchstart", handleClickOutside);
    };
  }, []);

  const handleBaseClick = (e: React.MouseEvent) => {
    // If not hovered (mobile tap), prevent navigation and show popover
    if (!isHovered) {
      e.preventDefault();
      setIsHovered(true);
    }
    // If already hovered, it acts as a normal link
  };

  return (
    <div
      ref={containerRef}
      className="flex flex-col justify-center items-center mt-8 pb-4 relative"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {isHovered && (
        <div
          className="absolute bottom-full mb-3 w-64 p-4 rounded-[20px] border border-white/50 bg-white/70 backdrop-blur-xl shadow-xl z-50 text-center animate-fade-in transition-all duration-300"
        >
          <div className="absolute -bottom-2 left-1/2 -translate-x-1/2 w-4 h-4 bg-white/70 border-b border-r border-white/50 transform rotate-45 backdrop-blur-xl z-40"></div>

          <div className="relative z-50">
            <div className="flex justify-center mb-2">
              <div className="w-10 h-10 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white shadow-md">
                <span className="text-lg">⚡</span>
              </div>
            </div>

            <h4 className="text-sm font-bold text-gray-900 font-outfit mb-1">
              Built with OneHumanCorp
            </h4>
            <p className="text-xs text-gray-600 mb-3 leading-relaxed">
              The AI-powered work assistant for modern owners and operators.
            </p>

            <a
              href={referralUrl}
              className="block w-full py-2 px-4 rounded-xl bg-gray-900 text-white text-xs font-semibold tracking-wide hover:bg-gray-800 transition-colors shadow-sm"
            >
              Create Your Own
            </a>
          </div>
        </div>
      )}

      <a
        href={referralUrl}
        onClick={handleBaseClick}
        className="group flex items-center gap-2 px-4 py-2 rounded-full border border-gray-200 bg-white/50 backdrop-blur-md hover:bg-white/80 hover:shadow-sm transition-all text-xs font-semibold text-gray-500 hover:text-indigo-600 uppercase tracking-widest font-outfit z-10 relative"
      >
        <span className="text-yellow-400 group-hover:scale-110 transition-transform">⚡</span>
        Powered by OHC
      </a>
    </div>
  );
}
