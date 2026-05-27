"use client";

import React from 'react';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

export function SkeletonBlock() {
  return (
    <div className="w-full p-6 animate-pulse">
      <div className="h-40 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[16px] mb-4 border border-white/50 dark:border-white/10" />
      <div className="h-4 w-3/4 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[8px] mb-2 border border-white/50 dark:border-white/10" />
      <div className="h-4 w-1/2 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[8px] border border-white/50 dark:border-white/10" />
    </div>
  );
}

export function ActionSheet({ isOpen, onClose, title, children }: { isOpen: boolean; onClose: () => void; title: string; children: React.ReactNode }) {
  if (!isOpen) return null;
  return (
    <div className="absolute inset-0 z-[100] flex flex-col justify-end">
      <div className="absolute inset-0 bg-black/40 backdrop-blur-sm" onClick={onClose} />
      <div className="bg-white/90 dark:bg-[#16161a]/90 backdrop-blur-xl border-t border-white/40 dark:border-white/10 w-full rounded-t-[16px] p-6 shadow-2xl animate-slide-up relative z-10">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">{title}</h2>
          <button onClick={onClose} className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 rounded-full hover:bg-white/40 dark:hover:bg-black/40 transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>
        {children}
      </div>
    </div>
  );
}

export function DraggableBlock({
  children,
  onDragStart,
  onDragOver,
  onDragEnd,
  onDragEnter,
  onMoveUp,
  onMoveDown,
  isSelected,
  onClick
}: {
  children: React.ReactNode;
  onDragStart: (e: React.TouchEvent | React.DragEvent) => void;
  onDragOver: (e: React.TouchEvent | React.DragEvent) => void;
  onDragEnter?: (e: React.DragEvent) => void;
  onDragEnd: (e: React.TouchEvent | React.DragEvent) => void;
  onMoveUp?: (e: React.MouseEvent) => void;
  onMoveDown?: (e: React.MouseEvent) => void;
  isSelected: boolean;
  onClick: () => void;
}) {
  return (
    <div
      draggable
      className={`relative group transition-all duration-200 cursor-move ${isSelected ? 'ring-2 ring-blue-500 z-10 shadow-lg scale-[1.02]' : 'hover:ring-1 hover:ring-blue-300'}`}
      onDragStart={onDragStart as (e: React.DragEvent) => void}
      onDragOver={(e) => {
        e.preventDefault();
        onDragOver(e);
      }}
      onDragEnter={onDragEnter}
      onDragEnd={onDragEnd as (e: React.DragEvent) => void}
      onTouchStart={onDragStart as (e: React.TouchEvent) => void}
      onTouchMove={onDragOver as (e: React.TouchEvent) => void}
      onTouchEnd={onDragEnd as (e: React.TouchEvent) => void}
      onClick={onClick}
    >
      {isSelected && (
        <div className="absolute -top-3 left-1/2 -translate-x-1/2 bg-blue-500 text-white text-[10px] font-bold px-2 py-0.5 rounded-full shadow-sm z-50 flex items-center gap-2">
          <span>DRAG TO REORDER</span>
          {onMoveUp && (
            <button
              onClick={(e) => { e.stopPropagation(); onMoveUp(e); }}
              className="px-1 hover:bg-blue-600 rounded bg-blue-500"
            >
              ↑
            </button>
          )}
          {onMoveDown && (
            <button
              onClick={(e) => { e.stopPropagation(); onMoveDown(e); }}
              className="px-1 hover:bg-blue-600 rounded bg-blue-500"
            >
              ↓
            </button>
          )}
        </div>
      )}
      {children}
    </div>
  );
}

export function QRCode({ value }: { value: string }) {
  return (
    <div className="bg-white/40 dark:bg-black/20 backdrop-blur-md p-4 rounded-[16px] shadow-sm border border-white/50 dark:border-white/10 inline-block">
      <svg className="w-32 h-32 rounded-[12px]" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect width="100" height="100" rx="12" fill="white" className="dark:fill-[#1D1D1F]"/>
        <rect x="10" y="10" width="20" height="20" fill="black"/>
        <rect x="15" y="15" width="10" height="10" fill="white"/>
        <rect x="70" y="10" width="20" height="20" fill="black"/>
        <rect x="75" y="15" width="10" height="10" fill="white"/>
        <rect x="10" y="70" width="20" height="20" fill="black"/>
        <rect x="15" y="75" width="10" height="10" fill="white"/>
        <rect x="40" y="40" width="20" height="20" fill="black" className="dark:fill-[#F5F5F7]"/>
        <rect x="45" y="45" width="10" height="10" fill="white" className="dark:fill-[#1D1D1F]"/>
        {/* Random dots to look like QR */}
        <rect x="40" y="10" width="5" height="5" fill="black" className="dark:fill-[#F5F5F7]"/>
        <rect x="10" y="40" width="5" height="5" fill="black" className="dark:fill-[#F5F5F7]"/>
        <rect x="70" y="40" width="5" height="5" fill="black" className="dark:fill-[#F5F5F7]"/>
        <rect x="40" y="70" width="5" height="5" fill="black" className="dark:fill-[#F5F5F7]"/>
        <rect x="60" y="60" width="10" height="10" fill="black" className="dark:fill-[#F5F5F7]"/>
        <rect x="80" y="80" width="10" height="10" fill="black" className="dark:fill-[#F5F5F7]"/>
      </svg>
    </div>
  );
}

export function SmartBlock({ type, props }: { type: string; props: any }) {
  if (type === "Hero") {
    return (
      <div className="relative w-full overflow-hidden bg-white/20 dark:bg-black/20 min-w-[375px]">
        <div
          className="absolute inset-0 bg-cover bg-center opacity-90"
          style={{ backgroundImage: `url(${props.image})` }}
        >
          <div className="absolute inset-0 bg-black/40 backdrop-blur-[2px]" />
        </div>
        <div className="relative z-10 p-6 flex flex-col items-center justify-center min-h-[300px] text-center text-white m-4 rounded-[16px] backdrop-blur-md bg-white/20 border border-white/40 shadow-lg">
          <h1 className="text-3xl font-bold font-outfit mb-3 tracking-tight">{props.headline}</h1>
          <p className="text-sm font-inter opacity-90 max-w-[280px]">{props.copy}</p>
        </div>
      </div>
    );
  }

  if (type === "Catalog") {
    return (
      <div className="p-6 bg-transparent font-inter min-w-[375px]">
        <h2 className="text-xl font-bold font-outfit mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] border-b border-white/40 dark:border-white/10 pb-2">Our Services</h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          {props.items.map((item: any, i: number) => (
            <div key={i} className="backdrop-blur-md bg-white/40 dark:bg-black/20 border border-white/50 dark:border-white/10 shadow-sm p-4 rounded-[16px] flex flex-col">
              <div className="flex justify-between items-start mb-1">
                <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">{item.name}</h3>
                <span className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-white/10 backdrop-blur-sm px-2 py-1 rounded-[8px] text-sm">{item.price}</span>
              </div>
              <p className="text-sm text-gray-600 dark:text-[#A1A1A6] leading-relaxed">{item.description}</p>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (type === "Booking") {
    return (
      <div className="p-6 bg-transparent font-inter min-w-[375px]">
        <div className="backdrop-blur-md bg-white/40 dark:bg-black/20 border border-white/50 dark:border-white/10 shadow-sm p-5 rounded-[16px] text-center">
          <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">{props.title}</h2>
          <p className="text-sm text-gray-600 dark:text-[#A1A1A6] mb-4">{props.availability}</p>
          <button className="w-full bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white font-semibold py-3 rounded-[8px] shadow-md hover:shadow-lg active:scale-[0.98] transition-all">
            Select Time
          </button>
        </div>
      </div>
    );
  }

  if (type === "Referral") {
    return (
      <div className="p-6 bg-gradient-to-br from-[#0066FF]/10 to-[#00C24B]/10 font-inter text-center border-t border-b border-white/40 dark:border-white/10 my-4 shadow-sm backdrop-blur-sm">
        <h2 className="text-xl font-bold font-outfit mb-2 text-[#1D1D1F] dark:text-[#F5F5F7]">{props.offerTitle || "Refer a Friend & Earn"}</h2>
        <p className="text-sm text-gray-700 dark:text-[#A1A1A6] mb-5">{props.offerDescription || "Get 20% off your next purchase when a friend buys from us!"}</p>

        <div className="flex gap-3 justify-center">
          <a
            href={`https://wa.me/?text=${encodeURIComponent(`Check out this store and get a discount! ${props.url || 'https://ohc.store'}`)}`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex-1 bg-gradient-to-r from-[#34C759] to-[#2eb350] text-white flex items-center justify-center gap-2 p-3 rounded-[8px] font-semibold text-sm shadow-sm hover:shadow-md hover:scale-[1.02] transition-all max-w-[140px]"
          >
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
            WhatsApp
          </a>
          <a
            href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Check out this store and get a discount! ${props.url || 'https://ohc.store'}`)}`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex-1 bg-black dark:bg-white text-white dark:text-black flex items-center justify-center gap-2 p-3 rounded-[8px] font-semibold text-sm shadow-sm hover:shadow-md hover:scale-[1.02] transition-all max-w-[140px]"
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
      <div className="p-6 bg-white/40 dark:bg-black/40 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] font-inter text-center border-y border-white/50 dark:border-white/10">
        <h2 className="text-lg font-bold font-outfit mb-4">Get in Touch</h2>
        <div className="space-y-2 text-sm text-gray-700 dark:text-[#A1A1A6]">
          <p>Email: <a href={`mailto:${props.email}`} className="text-[#0066FF] hover:underline">{props.email}</a></p>
          <p>Phone: <a href={`tel:${props.phone}`} className="text-[#0066FF] hover:underline">{props.phone}</a></p>
        </div>
      </div>
    );
  }

  if (type === "PoweredBy") {
    const tenantId = props.tenantId || "storefront";
    return (
      <div className="py-6 bg-transparent flex flex-col items-center justify-center border-t border-white/40 dark:border-white/10 mt-6">
        <a
          href={`ohc://join?ref=${tenantId}`}
          className="group flex items-center gap-2 text-sm text-gray-500 dark:text-[#A1A1A6] hover:text-[#1D1D1F] dark:hover:text-white transition-colors"
        >
          <span className="font-inter">Powered by</span>
          <span className="font-outfit font-bold tracking-tight">OHC</span>
          <svg className="w-4 h-4 opacity-0 -ml-2 group-hover:opacity-100 group-hover:ml-0 transition-all text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
          </svg>
        </a>
      </div>
    );
  }

  return null;
}
