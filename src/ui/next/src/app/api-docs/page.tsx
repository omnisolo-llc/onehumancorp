"use client";

import React, { useEffect, useState } from 'react';
import dynamic from 'next/dynamic';
import 'swagger-ui-react/swagger-ui.css';
import Link from 'next/link';

const SwaggerUI = dynamic(() => import('swagger-ui-react'), { ssr: false });

export default function ApiDocsPage() {
  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-5xl mx-auto">
        <div className="mb-8">
            <Link href="/help" className="text-blue-600 font-medium hover:underline inline-flex items-center">
                <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
                Back to Help Center
            </Link>
        </div>

        <article className="bg-white/80 backdrop-blur-[20px] saturate-200 p-8 sm:p-12 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] border border-white/60">
            <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-4 tracking-tight">API Documentation</h1>
            <p className="text-gray-600 mb-8 font-inter">
               Welcome to the OneHumanCorp API documentation. These endpoints allow you to build custom integrations.
            </p>
            <div className="swagger-container">
              <SwaggerUI url="/openapi.json" />
            </div>
        </article>
      </div>

      <style dangerouslySetInnerHTML={{ __html: `
        .swagger-container .swagger-ui {
          font-family: 'Inter', sans-serif !important;
        }
        .swagger-container .swagger-ui .info {
          margin: 0;
        }
        .swagger-container .swagger-ui .info .title {
           font-family: 'Outfit', sans-serif !important;
        }
      `}} />
    </div>
  );
}