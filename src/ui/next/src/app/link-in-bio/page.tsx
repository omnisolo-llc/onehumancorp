"use client";

import React, { useState, useEffect } from 'react';

export default function LinkInBioPage() {
  const [businessName, setBusinessName] = useState('My Business');
  const [description, setDescription] = useState('Welcome to my official page. Book a service or view my latest products below!');
  const [links, setLinks] = useState([
    { title: 'Book a Lesson', url: '/booking', type: 'primary' },
    { title: 'View Menu', url: '/store', type: 'secondary' },
    { title: 'Contact Me', url: '/contact', type: 'secondary' },
  ]);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedName = localStorage.getItem('business_name');
      if (storedName) {
        setBusinessName(storedName);
      }
    }
  }, []);

  return (
    <div className="flex flex-col min-h-screen items-center py-12 px-6 font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <div className="w-full max-w-sm flex flex-col items-center gap-6">
        <div className="w-24 h-24 rounded-full bg-gradient-to-tr from-indigo-500 to-purple-500 shadow-xl flex items-center justify-center text-3xl text-white font-bold mb-2">
          {businessName.substring(0, 2).toUpperCase()}
        </div>

        <div className="text-center">
            <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">{businessName}</h1>
            <p className="text-sm text-gray-600 leading-relaxed max-w-[280px] mx-auto">{description}</p>
        </div>

        <div className="w-full flex flex-col gap-4 mt-4">
            {links.map((link, idx) => (
                <a
                    key={idx}
                    href={link.url}
                    className={`w-full py-4 px-6 rounded-2xl font-semibold text-center shadow-sm transition-all hover:scale-[1.02] active:scale-[0.98] ${link.type === 'primary' ? 'bg-gray-900 text-white hover:bg-black' : 'bg-white text-gray-900 border border-gray-100 hover:border-gray-200'}`}
                    style={link.type === 'secondary' ? { background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)' } : {}}
                >
                    {link.title}
                </a>
            ))}
        </div>

        <div className="mt-8 pt-8 w-full border-t border-gray-200/50 text-center">
            <a href="https://ohc.store/join" className="text-xs font-semibold text-gray-400 uppercase tracking-widest hover:text-gray-600 transition-colors">
                ⚡ Powered by OHC
            </a>
        </div>
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
