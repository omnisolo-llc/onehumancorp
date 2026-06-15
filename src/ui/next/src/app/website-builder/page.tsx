"use client";

import React, { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from "@/components/ui/card";
import { Loader2, Wand2, Globe, Layout, Palette, Code, CheckCircle, AlertCircle } from "lucide-react";

export default function WebsiteBuilder() {
  const [prompt, setPrompt] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<any>(null);
  const [status, setStatus] = useState<string>("");

  const handleGenerate = async () => {
    if (!prompt) return;

    setIsLoading(true);
    setError(null);
    setResult(null);
    setStatus("Analyzing prompt...");

    try {
      // Simulate steps for UI feedback
      setTimeout(() => setStatus("Drafting layout structure..."), 1500);
      setTimeout(() => setStatus("Generating copy and selecting images..."), 3000);
      setTimeout(() => setStatus("Applying premium styles..."), 4500);

      const response = await fetch('/api/growth/zero_click_generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ prompt }),
      });

      if (!response.ok) {
        throw new Error(`Failed to generate website: ${response.statusText}`);
      }

      const data = await response.json();
      setResult(data);
      setStatus("Website generated successfully!");
    } catch (err: any) {
      console.error("Instant Build Error:", err);
      setError(err.message || "An unexpected error occurred during generation.");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container mx-auto p-6 max-w-5xl">
      <div className="mb-8">
        <h1 className="text-3xl font-bold tracking-tight mb-2 flex items-center gap-2">
          <Wand2 className="h-8 w-8 text-blue-600" />
          Instant Website Builder
        </h1>
        <p className="text-muted-foreground text-lg">
          Describe your business, and our AI agent will instantly generate a fully functional, premium website.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="md:col-span-2">
          <Card className="h-full border-blue-100 shadow-md">
            <CardHeader>
              <CardTitle>What do you want to build?</CardTitle>
              <CardDescription>Be as specific or as general as you like.</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex flex-col gap-4">
                <textarea
                  className="w-full min-h-[150px] p-4 rounded-md border border-input bg-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                  placeholder="e.g., A premium bakery in Brooklyn specializing in sourdough bread and custom wedding cakes. We need a way for customers to pre-order for weekend pickup."
                  value={prompt}
                  onChange={(e) => setPrompt(e.target.value)}
                  disabled={isLoading}
                />
              </div>
            </CardContent>
            <CardFooter className="flex justify-between items-center bg-slate-50/50 rounded-b-xl border-t p-4">
              <div className="text-sm text-muted-foreground flex items-center gap-2">
                {isLoading && (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin text-blue-600" />
                    {status}
                  </>
                )}
                {error && (
                  <span className="text-red-500 flex items-center gap-1 font-medium">
                    <AlertCircle className="h-4 w-4" /> Failed to launch
                  </span>
                )}
              </div>
              <Button
                onClick={handleGenerate}
                disabled={!prompt || isLoading}
                className="bg-blue-600 hover:bg-blue-700"
                size="lg"
              >
                {isLoading ? "Building..." : "Generate Website"}
              </Button>
            </CardFooter>
          </Card>
        </div>

        <div className="space-y-4">
          <Card className="bg-slate-50 border-none shadow-sm">
            <CardHeader className="pb-2">
              <CardTitle className="text-lg">What the Agent Does</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4 text-sm">
              <div className="flex gap-3">
                <div className="mt-0.5 bg-blue-100 p-1.5 rounded-full text-blue-700 h-fit">
                  <Layout className="h-4 w-4" />
                </div>
                <div>
                  <p className="font-medium">Structure & Layout</p>
                  <p className="text-muted-foreground">Designs responsive pages optimized for conversion.</p>
                </div>
              </div>
              <div className="flex gap-3">
                <div className="mt-0.5 bg-blue-100 p-1.5 rounded-full text-blue-700 h-fit">
                  <Palette className="h-4 w-4" />
                </div>
                <div>
                  <p className="font-medium">Copy & Images</p>
                  <p className="text-muted-foreground">Writes professional copy and selects fitting stock photography.</p>
                </div>
              </div>
              <div className="flex gap-3">
                <div className="mt-0.5 bg-blue-100 p-1.5 rounded-full text-blue-700 h-fit">
                  <Code className="h-4 w-4" />
                </div>
                <div>
                  <p className="font-medium">Functional Setup</p>
                  <p className="text-muted-foreground">Configures contact forms, service lists, and basic SEO.</p>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>

      {error && (
        <Card className="border-red-200 bg-red-50 mb-8">
          <CardHeader>
            <CardTitle className="text-red-700 flex items-center gap-2">
              <AlertCircle className="h-5 w-5" />
              Generation Failed
            </CardTitle>
          </CardHeader>
          <CardContent className="text-red-600">
            {error}
          </CardContent>
          <CardFooter>
            <Button variant="outline" onClick={() => setError(null)} className="border-red-200 text-red-700 hover:bg-red-100">
              Dismiss
            </Button>
          </CardFooter>
        </Card>
      )}

      {result && (
        <Card className="border-green-200 bg-green-50/30 overflow-hidden border-2 shadow-lg">
          <div className="bg-green-600 p-4 text-white flex justify-between items-center">
            <h2 className="text-xl font-semibold flex items-center gap-2">
              <CheckCircle className="h-6 w-6" />
              Your Website is Ready!
            </h2>
            <Button variant="secondary" size="sm" className="font-semibold">
              <Globe className="h-4 w-4 mr-2" />
              View Live Site
            </Button>
          </div>
          <CardContent className="p-6">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
              <div>
                <h3 className="font-medium text-lg mb-4 border-b pb-2">Generated Details</h3>
                <dl className="space-y-3 text-sm">
                  <div className="grid grid-cols-3 gap-2">
                    <dt className="text-muted-foreground font-medium">Business Name:</dt>
                    <dd className="col-span-2 font-medium">{result.business_name || 'Not specified'}</dd>
                  </div>
                  <div className="grid grid-cols-3 gap-2">
                    <dt className="text-muted-foreground font-medium">Theme:</dt>
                    <dd className="col-span-2">{result.theme_preference || 'Default Premium'}</dd>
                  </div>
                  <div className="grid grid-cols-3 gap-2">
                    <dt className="text-muted-foreground font-medium">Pages Created:</dt>
                    <dd className="col-span-2">{result.suggested_pages?.join(', ') || 'Home, About, Contact'}</dd>
                  </div>
                  <div className="grid grid-cols-3 gap-2">
                    <dt className="text-muted-foreground font-medium">Site ID:</dt>
                    <dd className="col-span-2 font-mono text-xs bg-slate-100 p-1 rounded inline-block">{result.site_id || 'generated-12345'}</dd>
                  </div>
                </dl>

                <div className="mt-6 space-y-3">
                  <Button className="w-full justify-start" variant="outline">
                    <Layout className="mr-2 h-4 w-4" /> Open Site Editor
                  </Button>
                  <Button className="w-full justify-start" variant="outline">
                    <Palette className="mr-2 h-4 w-4" /> Customize Theme
                  </Button>
                </div>
              </div>

              <div className="bg-slate-100 rounded-lg border shadow-inner flex items-center justify-center min-h-[300px] relative overflow-hidden">
                <div className="absolute top-0 w-full bg-slate-800 h-6 flex items-center px-2 gap-1.5">
                  <div className="w-2.5 h-2.5 rounded-full bg-red-400"></div>
                  <div className="w-2.5 h-2.5 rounded-full bg-amber-400"></div>
                  <div className="w-2.5 h-2.5 rounded-full bg-green-400"></div>
                  <div className="ml-4 bg-slate-700 h-3 w-1/2 rounded-full opacity-50"></div>
                </div>
                <div className="p-6 text-center mt-6">
                  <Globe className="h-16 w-16 text-slate-300 mx-auto mb-4" />
                  <p className="text-slate-500 font-medium">Live Preview Unavailable in Demo</p>
                  <p className="text-xs text-slate-400 mt-2">Click "View Live Site" to open in a new tab</p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
