"use client";

import React from 'react';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

export function SmartBlock({ type, props }: { type: string; props: any }) {
  if (type === "Hero") {
    return (
      <div className="relative w-full overflow-hidden bg-white">
        <div
          className="absolute inset-0 bg-cover bg-center opacity-90"
          style={{ backgroundImage: `url(${props.image})` }}
        >
          <div className="absolute inset-0 bg-black bg-opacity-40" />
        </div>
        <div className="relative z-10 p-6 flex flex-col items-center justify-center min-h-[300px] text-center text-white  glassmorphism">
          <h1 className="text-3xl font-bold font-outfit mb-3 tracking-tight">{props.headline}</h1>
          <p className="text-sm font-inter opacity-90 max-w-[280px]">{props.copy}</p>
        </div>
      </div>
    );
  }

  if (type === "Catalog") {
    return (
      <div className="p-6 bg-gray-50 font-inter">
        <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900 border-b pb-2">Our Services</h2>
        <div className="space-y-4">
          {props.items.map((item: any, i: number) => (
            <div key={i} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col">
              <div className="flex justify-between items-start mb-1">
                <h3 className="font-semibold text-gray-900">{item.name}</h3>
                <span className="font-bold text-gray-900 bg-gray-100 px-2 py-1 rounded-md text-sm">{item.price}</span>
              </div>
              <p className="text-sm text-gray-500 leading-relaxed">{item.description}</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (type === "Booking") {
    return (
      <div className="p-6 bg-white font-inter">
        <div className="bg-blue-50 border border-blue-100 p-5 rounded-xl text-center">
          <h2 className="text-lg font-bold font-outfit text-blue-900 mb-2">{props.title}</h2>
          <p className="text-sm text-blue-700 mb-4">{props.availability}</p>
          <button className="w-full bg-blue-600 text-white font-semibold py-3 rounded-lg shadow-sm active:scale-[0.98] transition-transform">
            Select Time
          </button>
        </div>
      </div>
    );
  }

  if (type === "Referral") {
    return (
      <div className="p-6 bg-gradient-to-br from-indigo-50 to-purple-50 font-inter text-center border-t border-b border-indigo-100 my-4 shadow-sm">
        <h2 className="text-xl font-bold font-outfit mb-2 text-indigo-900">{props.offerTitle || "Refer a Friend & Earn"}</h2>
        <p className="text-sm text-indigo-700 mb-5">{props.offerDescription || "Get 20% off your next purchase when a friend buys from us!"}</p>

        <div className="flex gap-3 justify-center">
          <a
            href={`https://wa.me/?text=${encodeURIComponent(`Check out this store and get a discount! ${props.url || 'https://ohc.store'}`)}`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex-1 bg-[#25D366] text-white flex items-center justify-center gap-2 p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all max-w-[140px]"
          >
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
            WhatsApp
          </a>
          <a
            href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Check out this store and get a discount! ${props.url || 'https://ohc.store'}`)}`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex-1 bg-black text-white flex items-center justify-center gap-2 p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all max-w-[140px]"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
            Share
          </a>
        </div>
      </div>
    );
  }

  if (type === "Contact") {
    return (
      <div className="p-6 bg-gray-900 text-white font-inter text-center">
        <h2 className="text-lg font-bold font-outfit mb-4">Get in Touch</h2>
        <div className="space-y-2 text-sm text-gray-300">
          <p>Email: <a href={`mailto:${props.email}`} className="text-blue-400">{props.email}</a></p>
          <p>Phone: <a href={`tel:${props.phone}`} className="text-blue-400">{props.phone}</a></p>
        </div>
      </div>
    );
  }

  if (type === "PoweredBy") {
    const tenantId = props.tenantId || "storefront";
    return (
      <div className="py-8 bg-white flex flex-col items-center justify-center border-t border-gray-100 font-inter">
        <div className="mb-2 text-xs font-semibold text-gray-400 uppercase tracking-wider">Start Your Own Store</div>
        <a
          href={`ohc://join?ref=${tenantId}`}
          className="group relative flex items-center gap-3 px-6 py-3 bg-gray-50 hover:bg-white rounded-2xl border border-gray-200 hover:border-blue-200 shadow-sm hover:shadow-md transition-all duration-300"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-blue-50 to-purple-50 opacity-0 group-hover:opacity-100 rounded-2xl transition-opacity duration-300 pointer-events-none"></div>
          <div className="w-8 h-8 rounded-full bg-gradient-to-br from-blue-600 to-purple-600 flex items-center justify-center shadow-inner relative z-10">
             <span className="text-white font-outfit font-bold text-xs">OHC</span>
          </div>
          <div className="flex flex-col relative z-10">
            <span className="text-[11px] font-medium text-gray-500 leading-tight">Powered by</span>
            <span className="font-outfit font-bold text-sm text-gray-900 leading-tight group-hover:text-blue-700 transition-colors">One Human Corp</span>
          </div>
          <svg className="w-5 h-5 ml-1 text-gray-400 group-hover:text-blue-600 group-hover:translate-x-1 transition-all relative z-10" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
        </a>
      </div>
    );
  }

  return null;
}
