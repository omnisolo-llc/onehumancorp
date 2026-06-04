"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function BrandVoiceTuning() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);

  const handleSelection = async (option: string, text: string) => {
    setLoading(true);
    try {
      await fetch(`/api/brand-voice/ab-test`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${localStorage.getItem('token') || ''}`, // Assuming token based auth
        },
        body: JSON.stringify({
          scenario: "A customer asks about shipping.",
          selected_option: option,
          selected_text: text,
        }),
      });
      router.push("/dashboard");
    } catch (e) {
      console.error(e);
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen p-4 bg-slate-50">
      <div className="w-full max-w-[375px] space-y-6">
        <div className="text-center">
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Let's teach your AI how you sound.</h1>
          <p className="mt-2 text-sm text-slate-500">Pick the response that sounds most like you.</p>
        </div>

        <div className="space-y-4">
          <Card className="cursor-pointer hover:border-blue-500 transition-colors" onClick={() => handleSelection("A", "Hi! 🍰 We ship within 2 days! Let me know if you need it sooner! ✨")}>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium text-slate-500">Option A</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-slate-900">"Hi! 🍰 We ship within 2 days! Let me know if you need it sooner! ✨"</p>
            </CardContent>
          </Card>

          <Card className="cursor-pointer hover:border-blue-500 transition-colors" onClick={() => handleSelection("B", "Our standard processing time is 48 hours. Expedited shipping is available upon request.")}>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium text-slate-500">Option B</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-slate-900">"Our standard processing time is 48 hours. Expedited shipping is available upon request."</p>
            </CardContent>
          </Card>
        </div>

        {loading && (
          <div className="text-center text-sm text-slate-500">
            Saving your brand voice...
          </div>
        )}
      </div>
    </div>
  );
}
