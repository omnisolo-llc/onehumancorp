"use client";

import React, { useEffect, useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';
import { PoweredByOHC } from '../../components/PoweredByOHC';

function VCardContent() {
  const searchParams = useSearchParams();
  const [cardData, setCardData] = useState<{
    tenant: string;
    name: string;
    title: string;
    company: string;
    phone: string;
    email: string;
    website: string;
    linkedin: string;
    themeColor: string;
    removeBranding: boolean;
  } | null>(null);
  const [error, setError] = useState(false);

  useEffect(() => {
    const dataParam = searchParams.get('data');
    if (dataParam) {
      try {
        // Base64Url decode logic
        let base64Str = dataParam.replace(/-/g, '+').replace(/_/g, '/');
        while (base64Str.length % 4) {
          base64Str += '=';
        }
        const utf8Encoded = escape(atob(base64Str));
        const decodedData = JSON.parse(decodeURIComponent(utf8Encoded));
        setCardData(decodedData);
      } catch (err) {
        console.error("Failed to parse vcard data", err);
        setError(true);
      }
    } else {
        setError(true);
    }
  }, [searchParams]);

  const handleDownloadVCard = () => {
    if (!cardData) return;

    const vcardLines = [
      "BEGIN:VCARD",
      "VERSION:3.0",
      `N:;${cardData.name};;;`,
      `FN:${cardData.name}`,
      cardData.title ? `TITLE:${cardData.title}` : "",
      cardData.company ? `ORG:${cardData.company}` : "",
      cardData.phone ? `TEL;TYPE=WORK,VOICE:${cardData.phone}` : "",
      cardData.email ? `EMAIL;TYPE=PREF,INTERNET:${cardData.email}` : "",
      cardData.website ? `URL:${cardData.website}` : "",
      cardData.linkedin ? `URL;type=LinkedIn:${cardData.linkedin}` : "",
      "END:VCARD"
    ].filter(line => line !== "");

    const blob = new Blob([vcardLines.join("\n")], { type: 'text/vcard' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `${cardData.name.replace(/\s+/g, '_')}_Contact.vcf`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[#F5F5F7] font-inter">
        <p className="text-[#FF3B30] bg-red-50 p-4 rounded-xl">Error: Invalid or corrupted card data.</p>
      </div>
    );
  }

  if (!cardData) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[#F5F5F7] font-inter">
        <p className="text-gray-500 animate-pulse">Loading card...</p>
      </div>
    );
  }

  const { tenant, name, title, company, phone, email, website, linkedin, themeColor, removeBranding } = cardData;

  return (
    <div className="min-h-screen font-inter bg-gray-50 flex flex-col items-center justify-center p-4 py-12 custom-scrollbar">
      <div className="w-full max-w-sm">

        {/* Main Card */}
        <div className="bg-white rounded-[2.5rem] shadow-2xl overflow-hidden relative border border-gray-100 pb-8">
          {/* Header/Cover */}
          <div className="h-32 w-full relative" style={{ backgroundColor: themeColor, opacity: 0.85 }}>
            <div className="absolute inset-0 bg-gradient-to-b from-transparent to-black/20"></div>
          </div>

          {/* Avatar */}
          <div className="relative flex justify-center -mt-16 mb-4">
            <div
              className="w-32 h-32 rounded-full border-4 border-white flex items-center justify-center text-5xl font-bold text-white shadow-lg bg-gray-900"
              style={{ backgroundColor: themeColor }}
            >
              {name ? name.charAt(0).toUpperCase() : 'J'}
            </div>
          </div>

          <div className="px-6 text-center mb-8">
            <h1 className="text-3xl font-bold text-gray-900 mb-1 font-outfit tracking-tight">{name}</h1>
            <p className="text-base font-semibold mb-1" style={{ color: themeColor }}>{title}</p>
            <p className="text-sm text-gray-500 font-medium">{company}</p>
          </div>

          {/* Action Button */}
          <div className="px-6 mb-8">
            <button
              onClick={handleDownloadVCard}
              className="w-full py-4 rounded-2xl text-white font-bold text-lg shadow-lg hover:shadow-xl transition-all active:scale-[0.98] flex items-center justify-center gap-2"
              style={{ backgroundColor: themeColor }}
            >
              <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
              Save to Contacts
            </button>
          </div>

          {/* Contact Details */}
          <div className="px-6 space-y-4">
            {phone && (
              <a href={`tel:${phone.replace(/[^\d+]/g, '')}`} className="bg-gray-50 hover:bg-gray-100 p-4 rounded-2xl border border-gray-100 flex items-center gap-4 transition-colors group">
                <div className="w-12 h-12 rounded-full bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm flex items-center justify-center flex-shrink-0 group-hover:scale-110 transition-transform" style={{ color: themeColor }}>
                  📱
                </div>
                <div className="overflow-hidden">
                  <p className="text-xs text-gray-500 font-bold uppercase tracking-wider mb-0.5">Mobile</p>
                  <p className="text-base font-semibold text-gray-900 truncate">{phone}</p>
                </div>
              </a>
            )}

            {email && (
              <a href={`mailto:${email}`} className="bg-gray-50 hover:bg-gray-100 p-4 rounded-2xl border border-gray-100 flex items-center gap-4 transition-colors group">
                <div className="w-12 h-12 rounded-full bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm flex items-center justify-center flex-shrink-0 group-hover:scale-110 transition-transform" style={{ color: themeColor }}>
                  ✉️
                </div>
                <div className="overflow-hidden">
                  <p className="text-xs text-gray-500 font-bold uppercase tracking-wider mb-0.5">Email</p>
                  <p className="text-base font-semibold text-gray-900 truncate">{email}</p>
                </div>
              </a>
            )}

            {website && (
              <a href={website.startsWith('http') ? website : `https://${website}`} target="_blank" rel="noopener noreferrer" className="bg-gray-50 hover:bg-gray-100 p-4 rounded-2xl border border-gray-100 flex items-center gap-4 transition-colors group">
                <div className="w-12 h-12 rounded-full bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm flex items-center justify-center flex-shrink-0 group-hover:scale-110 transition-transform" style={{ color: themeColor }}>
                  🌐
                </div>
                <div className="overflow-hidden">
                  <p className="text-xs text-gray-500 font-bold uppercase tracking-wider mb-0.5">Website</p>
                  <p className="text-base font-semibold text-gray-900 truncate">{website}</p>
                </div>
              </a>
            )}

            {linkedin && (
              <a href={linkedin.startsWith('http') ? linkedin : `https://${linkedin}`} target="_blank" rel="noopener noreferrer" className="bg-gray-50 hover:bg-gray-100 p-4 rounded-2xl border border-gray-100 flex items-center gap-4 transition-colors group">
                <div className="w-12 h-12 rounded-full bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm flex items-center justify-center flex-shrink-0 group-hover:scale-110 transition-transform" style={{ color: themeColor }}>
                  💼
                </div>
                <div className="overflow-hidden">
                  <p className="text-xs text-gray-500 font-bold uppercase tracking-wider mb-0.5">LinkedIn</p>
                  <p className="text-base font-semibold text-gray-900 truncate">Connect</p>
                </div>
              </a>
            )}
          </div>
        </div>

        {/* Viral Growth Loop Footer */}
        {!removeBranding && (
          <div className="mt-8 text-center animate-fade-in flex flex-col items-center">
            <PoweredByOHC tenantId={tenant} />
            <a
              href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}&source=digital_business_card`}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex flex-col items-center gap-1 group mt-3"
            >
              <span className="text-sm font-bold text-gray-500 group-hover:text-indigo-600 transition-colors">
                Create your own free digital business card
              </span>
            </a>
          </div>
        )}

      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.5s ease-out forwards; }
      `}} />
    </div>
  );
}

export default function DigitalBusinessCardViewPage() {
  return (
    <Suspense fallback={
      <div className="min-h-screen flex items-center justify-center bg-gray-50 font-inter">
        <p className="text-gray-500 font-bold animate-pulse">Loading Card...</p>
      </div>
    }>
      <VCardContent />
    </Suspense>
  );
}
