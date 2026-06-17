"use client";

import React, { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';

export default function LinkInBioPublicPage() {
    const params = useParams();
    const tenantId = typeof params?.tenant === 'string' ? params.tenant : 'my-store';

    // In a real implementation, we would fetch data from the backend here.
    // Since this is a lightweight platform, we'll try to load from localStorage
    // to simulate the saved data (if running locally). In prod, it should fetch.
    const [storeName, setStoreName] = useState('My Store');
    const [bio, setBio] = useState('Welcome to my storefront!');
    const [links, setLinks] = useState<any[]>([
        { id: '1', title: 'Visit My Store', url: '/website-builder' },
        { id: '2', title: 'Book an Appointment', url: '/booking' },
    ]);
    const [theme, setTheme] = useState('gradient');
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        // Attempt to load from localStorage to simulate persistence
        const fetchBioData = async () => {
            try {
                if (typeof localStorage !== 'undefined') {
                    const savedData = localStorage.getItem(`ohc_bio_${tenantId}`);
                    if (savedData) {
                        const parsed = JSON.parse(savedData);
                        setStoreName(parsed.storeName || 'My Store');
                        setBio(parsed.bio || 'Welcome to my storefront!');
                        setLinks(parsed.links || []);
                        setTheme(parsed.theme || 'gradient');
                    } else {
                        // Fallback generic data
                        const storedName = localStorage.getItem('business_name');
                        if (storedName) setStoreName(storedName);
                    }
                }
            } catch (e) {
                console.error("Error loading bio data:", e);
            } finally {
                setLoading(false);
            }
        };
        fetchBioData();
    }, [tenantId]);

    const getThemeStyles = () => {
        switch(theme) {
            case 'dark': return { background: '#1D1D1F', color: '#ffffff' };
            case 'light': return { background: '#ffffff', color: '#1D1D1F' };
            case 'purple': return { background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', color: '#ffffff' };
            case 'gradient': default: return { background: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)', color: '#1D1D1F' };
        }
    };

    if (loading) {
        return <div className="min-h-screen flex items-center justify-center font-inter">Loading...</div>;
    }

    return (
        <div className="min-h-screen flex justify-center font-inter" style={{ backgroundColor: theme === 'light' ? '#f3f4f6' : '#000' }}>
             <div className="w-full max-w-[480px] min-h-screen relative flex flex-col items-center shadow-2xl transition-all duration-300" style={getThemeStyles()}>
                 <div className="w-full h-full flex flex-col items-center overflow-y-auto pt-16 pb-12 px-6">

                     <div className="w-24 h-24 rounded-full bg-white/20 shadow-inner flex items-center justify-center backdrop-blur-md mb-6 mt-4 border border-white/30 text-4xl">
                         ✨
                     </div>

                     <h1 className="text-3xl font-bold font-outfit mb-3 text-center drop-shadow-sm">
                         {storeName}
                     </h1>

                     <p className="text-base font-medium opacity-90 text-center mb-10 max-w-xs drop-shadow-sm leading-relaxed">
                         {bio}
                     </p>

                     <div className="w-full flex flex-col gap-4">
                         {links.map((link: any) => (
                             <a
                                 key={link.id}
                                 href={link.url}
                                 target="_blank"
                                 rel="noopener noreferrer"
                                 className="w-full py-4 px-6 rounded-2xl text-center font-bold text-[15px] transition-transform hover:scale-[1.02] active:scale-95 shadow-sm"
                                 style={{
                                     background: theme === 'light' ? '#ffffff' : 'rgba(255, 255, 255, 0.15)',
                                     border: theme === 'light' ? '1px solid #e5e7eb' : '1px solid rgba(255, 255, 255, 0.3)',
                                     backdropFilter: 'blur(10px)',
                                     color: theme === 'light' ? '#111827' : '#ffffff'
                                 }}
                             >
                                 {link.title || 'Untitled Link'}
                             </a>
                         ))}
                     </div>

                     <div className="mt-auto pt-12 pb-6 w-full flex justify-center">
                         <a href={`https://ohc.store/join?ref=${tenantId}`} className="text-sm font-semibold tracking-wider uppercase opacity-70 hover:opacity-100 transition-opacity flex flex-col items-center gap-1">
                             ⚡ Powered by OHC
                         </a>
                     </div>
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
