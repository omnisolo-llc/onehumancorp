"use client";

import React from 'react';

export function SmartBlock({ type, props }: { type: string; props: any }) {
  if (type === "Hero") {
    return (
      <div className="relative w-full overflow-hidden bg-white dark:bg-[#16161A]">
        <div
          className="absolute inset-0 bg-cover bg-center opacity-90"
          style={{ backgroundImage: `url(${props.image})` }}
        >
          <div className="absolute inset-0 bg-black/40 dark:bg-black/60" />
        </div>
        <div className="relative z-10 p-6 flex flex-col items-center justify-center min-h-[300px] text-center text-white glassmorphism-header border-y-0 backdrop-blur-md">
          <h1 className="text-3xl font-bold font-outfit mb-3 tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]">{props.headline}</h1>
          <p className="text-sm font-inter opacity-90 max-w-[280px] text-gray-700 dark:text-gray-300">{props.copy}</p>
        </div>
      </div>
    );
  }

  if (type === "Catalog") {
    return (
      <div className="p-6 bg-gray-50 dark:bg-[#1D1D1F] font-inter">
        <h2 className="text-xl font-bold font-outfit mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] border-b border-gray-200 dark:border-gray-800 pb-2">Our Services</h2>
        <div className="space-y-4">
          {props.items.map((item: any, i: number) => (
            <div key={i} className="glassmorphism-input p-4 shadow-sm flex flex-col transition-transform duration-250 hover:scale-[1.02]">
              <div className="flex justify-between items-start mb-1">
                <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">{item.name}</h3>
                <span className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7] bg-gray-100/50 dark:bg-white/10 px-2 py-1 rounded-md text-sm backdrop-blur-md">{item.price}</span>
              </div>
              <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">{item.description}</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (type === "Booking") {
    return (
      <div className="p-6 bg-white dark:bg-[#16161A] font-inter">
        <div className="glassmorphism-container p-5 text-center transition-transform duration-250 hover:scale-[1.02]">
          <h2 className="text-lg font-bold font-outfit text-[#0071E3] dark:text-[#3AB0FF] mb-2">{props.title}</h2>
          <p className="text-sm text-gray-700 dark:text-gray-300 mb-4">{props.availability}</p>
          <button className="w-full bg-[#0071E3] text-white font-semibold py-3 glassmorphism-button shadow-sm active:scale-[0.98] transition-transform duration-250">
            Select Time
          </button>
        </div>
      </div>
    );
  }

  if (type === "Contact") {
    return (
      <div className="p-6 bg-[#1D1D1F] dark:bg-[#111111] text-[#F5F5F7] font-inter text-center border-t border-white/10">
        <h2 className="text-lg font-bold font-outfit mb-4">Get in Touch</h2>
        <div className="space-y-2 text-sm text-gray-300">
          <p>Email: <a href={`mailto:${props.email}`} className="text-[#0071E3] dark:text-[#3AB0FF] hover:underline">{props.email}</a></p>
          <p>Phone: <a href={`tel:${props.phone}`} className="text-[#0071E3] dark:text-[#3AB0FF] hover:underline">{props.phone}</a></p>
        </div>
      </div>
    );
  }

  return null;
}
