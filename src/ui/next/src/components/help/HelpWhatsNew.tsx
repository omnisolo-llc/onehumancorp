import React from 'react';
import { WithTooltip } from '../TooltipRegistry';

export function HelpWhatsNew() {
  return (
    <div className="backdrop-blur-[30px] saturate-[210%] bg-[rgba(255,255,255,0.65)] border-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)] p-4 rounded-xl border shadow-sm">
      <h3 className="font-bold font-outfit text-gray-900 mb-4 text-xl">What's New</h3>
      <div className="w-full aspect-video bg-gray-200 rounded-2xl mb-6 relative overflow-hidden border border-white/50 shadow-md flex items-center justify-center">
         <div className="w-full h-full bg-gradient-to-br from-blue-100 to-indigo-100 flex items-center justify-center text-blue-400">
           <svg className="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 002-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg>
         </div>
      </div>
      <div className="app-card border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 rounded-2xl shadow-sm mb-6 bg-[rgba(255,255,255,0.65)] dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%]">
        <span className="inline-block px-2 py-1 bg-blue-100 text-blue-700 text-xs font-bold rounded-md mb-2">LATEST</span>
        <h4 className="font-bold font-outfit text-gray-900 text-base mb-2">New AI Store Builder</h4>
        <p className="text-sm text-gray-600 leading-relaxed mb-4">You can now generate a complete storefront from just a short description of your business. Try it out in the Storefront Builder.</p>

        <WithTooltip id="changelog-nav-tooltip" defaultText="See what's new in the latest OneHumanCorp updates.">
          <a href="/changelog" className="inline-flex items-center text-blue-600 text-sm font-bold hover:text-blue-800 transition-colors bg-blue-50/80 px-4 py-2 rounded-xl min-h-[44px]">
            Read full release notes
            <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
          </a>
        </WithTooltip>
      </div>
    </div>
  );
}
