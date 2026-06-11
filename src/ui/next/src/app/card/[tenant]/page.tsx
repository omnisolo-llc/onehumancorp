"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";

interface DigitalCard {
  id: string;
  name: string;
  title: string;
  company: string;
  email: string;
  phone?: string;
  bio?: string;
  website?: string;
  theme: string;
}

export default function DigitalCardView() {
  const params = useParams();
  const id = params.tenant as string;
  const [card, setCard] = useState<DigitalCard | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    if (!id) return;

    fetch(`/api/v1/growth/digital-card/${id}`)
      .then((res) => {
        if (!res.ok) throw new Error("Not found");
        return res.json();
      })
      .then((data) => {
        setCard(data);
        setLoading(false);
      })
      .catch((err) => {
        console.error(err);
        setError(true);
        setLoading(false);
      });
  }, [id]);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-gray-500">Loading card...</div>
      </div>
    );
  }

  if (error || !card) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-red-500">Card not found</div>
      </div>
    );
  }

  const isDark = card.theme === "dark";
  const bgClass = isDark ? "bg-gray-900" : "bg-white";
  const textClass = isDark ? "text-white" : "text-gray-900";
  const subTextClass = isDark ? "text-gray-400" : "text-gray-500";
  const borderClass = isDark ? "border-gray-800" : "border-gray-200";
  const wrapperClass = isDark ? "bg-black" : "bg-gray-50";

  const handleDownloadVcard = () => {
    const vcard = `BEGIN:VCARD
VERSION:3.0
N:;${card.name};;;
FN:${card.name}
ORG:${card.company}
TITLE:${card.title}
EMAIL;type=INTERNET;type=WORK;type=pref:${card.email}
TEL;type=WORK;type=VOICE;type=pref:${card.phone || ""}
URL:${card.website || ""}
NOTE:${card.bio || ""}
END:VCARD`;

    const blob = new Blob([vcard], { type: "text/vcard" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${card.name.replace(/\s+/g, "_")}.vcf`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className={`min-h-screen flex flex-col justify-between ${wrapperClass} font-sans`}>
      <main className="flex-grow flex items-center justify-center p-4">
        <div className={`w-full max-w-sm rounded-2xl shadow-xl overflow-hidden ${bgClass} border ${borderClass}`}>
          <div className="p-8 text-center">
            <div className={`w-24 h-24 mx-auto rounded-full flex items-center justify-center text-3xl font-bold mb-4 ${isDark ? 'bg-gray-800 text-white' : 'bg-gray-100 text-gray-800'}`}>
              {card.name.charAt(0)}
            </div>
            <h1 className={`text-2xl font-bold mb-1 ${textClass}`}>{card.name}</h1>
            <p className={`text-lg mb-1 ${textClass}`}>{card.title}</p>
            <p className={`text-sm font-medium mb-6 ${subTextClass}`}>{card.company}</p>

            {card.bio && (
              <p className={`text-sm mb-6 ${textClass} italic`}>"{card.bio}"</p>
            )}

            <div className="space-y-4 text-left">
              <a href={`mailto:${card.email}`} className={`flex items-center p-3 rounded-xl border ${borderClass} hover:opacity-80 transition-opacity`}>
                <span className="mr-3 text-xl">📧</span>
                <span className={`text-sm font-medium ${textClass}`}>{card.email}</span>
              </a>

              {card.phone && (
                <a href={`tel:${card.phone}`} className={`flex items-center p-3 rounded-xl border ${borderClass} hover:opacity-80 transition-opacity`}>
                  <span className="mr-3 text-xl">📱</span>
                  <span className={`text-sm font-medium ${textClass}`}>{card.phone}</span>
                </a>
              )}

              {card.website && (
                <a href={card.website.startsWith('http') ? card.website : `https://${card.website}`} target="_blank" rel="noopener noreferrer" className={`flex items-center p-3 rounded-xl border ${borderClass} hover:opacity-80 transition-opacity`}>
                  <span className="mr-3 text-xl">🌐</span>
                  <span className={`text-sm font-medium ${textClass}`}>{card.website}</span>
                </a>
              )}
            </div>

            <button
              onClick={handleDownloadVcard}
              className={`mt-8 w-full py-3 px-4 rounded-xl font-bold text-white transition-opacity hover:opacity-90 ${isDark ? 'bg-blue-500' : 'bg-black'}`}
            >
              Save Contact
            </button>
          </div>
        </div>
      </main>

      <footer className="p-6 text-center">
        <a
          href="https://ohc.app/register?ref=digital_card"
          target="_blank"
          rel="noopener noreferrer"
          className={`inline-flex items-center text-sm font-medium ${subTextClass} hover:${textClass} transition-colors`}
        >
          Powered by <span className="font-bold ml-1">OHC</span>
        </a>
      </footer>
    </div>
  );
}
