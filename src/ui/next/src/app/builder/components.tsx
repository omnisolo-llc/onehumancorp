"use client";

import React from 'react';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

export function SmartBlock({ type, props }: { type: string; props: any }) {
  if (type === "HeroBlock") {
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
          <p className="text-sm font-inter opacity-90 max-w-[280px]">{props.subtitle}</p>
        </div>
      </div>
    );
  }

  if (type === "ProductGridBlock") {
    return (
      <div className="p-6 bg-gray-50 font-inter">
        <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900 border-b pb-2">Products & Services</h2>
        <div className="space-y-4">
          {props.items && props.items.map((item: any, i: number) => (
            <div key={i} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col">
              <div className="flex justify-between items-start mb-1">
                <h3 className="font-semibold text-gray-900">{typeof item === 'string' ? item : item.name}</h3>
                {item.price && <span className="font-bold text-gray-900 bg-gray-100 px-2 py-1 rounded-md text-sm">{item.price}</span>}
              </div>
              {item.description && <p className="text-sm text-gray-500 leading-relaxed">{item.description}</p>}
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (type === "ServiceBookingBlock" || type === "BookingCalendarBlock") {
    return (
      <div className="p-6 bg-white font-inter">
        <div className="bg-blue-50 border border-blue-100 p-5 rounded-xl text-center">
          <h2 className="text-lg font-bold font-outfit text-blue-900 mb-2">{props.title || "Book a Service"}</h2>
          <div className="space-y-2 mb-4 text-sm text-blue-700">
            {props.services && props.services.map((s: string, i: number) => (
               <p key={i}>• {s}</p>
            ))}
          </div>
          <button className="w-full bg-blue-600 text-white font-semibold py-3 rounded-lg shadow-sm active:scale-[0.98] transition-transform">
            Select Time
          </button>
        </div>
      </div>
    );
  }

  if (type === "TestimonialBlock") {
    return (
      <div className="p-6 bg-gray-900 text-white font-inter text-center">
        <h2 className="text-lg font-bold font-outfit mb-4">What our customers say</h2>
        <div className="space-y-4 text-sm text-gray-300">
          {props.testimonials && props.testimonials.map((t: string, i: number) => (
             <blockquote key={i} className="italic text-gray-400">"{t}"</blockquote>
          ))}
        </div>
      </div>
    );
  }

  if (type === "ContactFormBlock") {
    return (
      <div className="p-6 bg-gray-900 text-white font-inter text-center">
        <h2 className="text-lg font-bold font-outfit mb-4">Get in Touch</h2>
        <div className="space-y-2 text-sm text-gray-300">
          <p>Email: <a href={`mailto:${props.email}`} className="text-blue-400">{props.email || 'hello@example.com'}</a></p>
          <p>Phone: <a href={`tel:${props.phone}`} className="text-blue-400">{props.phone || '(555) 123-4567'}</a></p>
        </div>
      </div>
    );
  }

  return (
     <div className="p-6 text-center text-gray-500">
        Unknown block type: {type}
     </div>
  );
}
