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
        <div className="relative z-10 p-6 flex flex-col items-center justify-center min-h-[300px] text-center text-white backdrop-blur-sm bg-white/10 glassmorphism">
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

  if (type === "FooterBranding") {
    return (
      <div className="p-6 bg-gray-100 font-inter text-center border-t border-gray-200">
        <a
          href={`https://ohc.store?ref=${props.refId || 'poweredby'}`}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center justify-center gap-2 px-4 py-2 bg-white rounded-full shadow-sm hover:shadow-md transition-shadow border border-gray-200 text-gray-700 text-sm font-medium"
        >
          <span className="opacity-70">Powered by</span>
          <span className="font-bold font-outfit text-blue-600">OHC</span>
          <svg className="w-4 h-4 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </a>
      </div>
    );
  }

  return null;
}
