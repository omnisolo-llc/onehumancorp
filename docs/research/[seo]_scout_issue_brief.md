# Issue Brief: Autonomous SEO Optimization & Indexing Service

## Problem Statement
SMB websites often rank poorly on Google because owners do not understand SEO basics (meta tags, alt text, structured data). Learning SEO is too time-consuming, resulting in zero organic traffic.

## Research Report
A majority of small business websites lack basic structural SEO. Automated SEO tools exist as plugins (e.g., Yoast), but they still require manual user input. An autonomous system that evaluates content and automatically injects optimized meta tags, generates sitemaps, and requests indexing from Google Search Console removes this friction entirely.

## Design Doc
**Architecture:**
- Background crawler for tenant domains.
- SEO Metadata entity linked to pages/products.
- Integration with Google Search Console API.
**AI Integration:**
- NLP generation of concise meta descriptions and alt text for all uploaded images.

## Implementation Prompt
Build a service that automatically scans new products and pages, generating optimized meta titles, descriptions, and JSON-LD structured data. Automatically submit updated sitemaps to search engines. Acceptance criteria: Adding a product triggers generation of SEO metadata and a mock API call to update the sitemap.

## Priority
P3

## Estimated Scope
Large
