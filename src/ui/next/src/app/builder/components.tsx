"use client";

import React from 'react';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

export function SmartBlock({ type, props }: { type: string; props: any }) {
  if (type === "HeroBlock" || type === "Hero") {
    return (
      <div className="relative w-full overflow-hidden bg-white">
        <div
          className="absolute inset-0 bg-cover bg-center opacity-90"
          style={{ backgroundImage: `url(${props.image || 'https://images.unsplash.com/photo-1516734212186-a967f81ad0d7?auto=format&fit=crop&w=400&q=80'})` }}
        >
          <div className="absolute inset-0 bg-black bg-opacity-40" />
        </div>
        <div className="relative z-10 p-6 flex flex-col items-center justify-center min-h-[300px] text-center text-white backdrop-blur-sm bg-white/10 glassmorphism">
          <h1 className="text-3xl font-bold font-outfit mb-3 tracking-tight">{props.headline}</h1>
          <p className="text-sm font-inter opacity-90 max-w-[280px]">{props.subtitle || props.copy}</p>
        </div>
      </div>
    );
  }

  if (type === "ProductGridBlock") {
    return (
      <div className="p-6 bg-gray-50 font-inter">
        <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900 border-b pb-2">Products</h2>
        <div className="space-y-4">
          {props.items && props.items.map((item: any, i: number) => (
            <div key={i} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col">
              <div className="flex justify-between items-start mb-1">
                <h3 className="font-semibold text-gray-900">{typeof item === 'string' ? item : item.name}</h3>
                <span className="font-bold text-gray-900 bg-gray-100 px-2 py-1 rounded-md text-sm">{item.price || '$ Varies'}</span>
              </div>
              <p className="text-sm text-gray-500 leading-relaxed">{item.description || 'Available'}</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (type === "ServiceBookingBlock") {
    return (
      <div className="p-6 bg-gray-50 font-inter">
        <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900 border-b pb-2">Book Services</h2>
        <div className="space-y-4">
          {props.services && props.services.map((item: any, i: number) => (
            <div key={i} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col">
              <div className="flex justify-between items-start mb-1">
                <h3 className="font-semibold text-gray-900">{typeof item === 'string' ? item : item.name}</h3>
                <span className="font-bold text-gray-900 bg-gray-100 px-2 py-1 rounded-md text-sm">Contact us</span>
              </div>
              <p className="text-sm text-gray-500 leading-relaxed">Available</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (type === "TestimonialBlock") {
    return (
      <div className="p-6 bg-gray-50 font-inter">
        <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900 border-b pb-2">Testimonials</h2>
        <div className="space-y-4">
          {props.testimonials && props.testimonials.map((item: any, i: number) => (
            <div key={i} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col">
              <div className="flex justify-between items-start mb-1">
                <h3 className="font-semibold text-gray-900">Review</h3>
                <span className="font-bold text-gray-900 bg-gray-100 px-2 py-1 rounded-md text-sm">5 Stars</span>
              </div>
              <p className="text-sm text-gray-500 leading-relaxed">{item}</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (type === "PoweredBy") {
    return (
      <div className="py-6 bg-gray-50 flex flex-col items-center justify-center border-t border-gray-100">
        <a
          href="https://ohc.store"
          target="_blank"
          rel="noopener noreferrer"
          className="group flex items-center gap-2 text-sm text-gray-500 hover:text-gray-900 transition-colors"
        >
          <span className="font-inter">Powered by</span>
          <span className="font-outfit font-bold tracking-tight">OHC</span>
          <svg className="w-4 h-4 opacity-0 -ml-2 group-hover:opacity-100 group-hover:ml-0 transition-all text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
          </svg>
        </a>
      </div>
    );
  }

  return null;
}
