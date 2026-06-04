# [Architecture] Autonomous AI Product Photography Studio

## Problem Statement
Small business owners like Maya (baker) and Priya (boutique owner) struggle to create professional-quality product photos. They rely on their smartphone cameras, often resulting in poorly lit, distracting, or inconsistent images. Hiring a professional photographer or using complex editing software like Photoshop is too expensive and time-consuming. They need a simple, zero-touch way to upload a raw smartphone photo and have it automatically transformed into a clean, studio-quality product image suitable for an e-commerce storefront, Instagram, and promotional materials.

## Research Report
**Competitive Analysis:**
- **Shopify:** Provides basic image editing (cropping, resizing) and some third-party apps for background removal, but lacks a fully integrated, high-quality AI product photography suite out-of-the-box.
- **Canva / Photoroom:** Excellent standalone tools for AI background removal and scene generation, but require exporting and re-uploading to the storefront platform.
- **Wix / Squarespace:** Offer basic image adjustments, but no native AI scene generation for products.

**Market Needs:**
Visuals are critical for conversion. Solopreneurs need an "invisible photography studio." They should be able to snap a photo of a cake on a messy kitchen counter or a dress on a hanger, and have the system automatically remove the background, enhance the lighting, and place the product in a relevant, aesthetic, high-converting setting (e.g., a clean marble countertop for the cake, or a well-lit minimalist studio for the dress) with zero technical prompt engineering.

## Design Doc

### Architecture Diagram
```mermaid
graph TD;
    subgraph Mobile App
        Camera[Native Camera / Photo Picker] --> UploadService[Image Upload Service];
        UploadService --> LocalCache[(Local Image Cache)];
    end

    UploadService -- Raw Image --> API[OHC API Gateway];
    API --> BackgroundJobQueue[AI Job Queue (Postgres)];

    subgraph AI Marketing Department
        BackgroundJobQueue --> PhotoAgent[AI Photography Agent];
        PhotoAgent --> BackgroundRemoval[Background Removal API (e.g., rembg/Cloudflare)];
        BackgroundRemoval --> SceneGen[GenAI Image Model (e.g., Stable Diffusion / Midjourney API)];
        SceneGen --> ImageOptimizer[WebP Optimizer & Resizer];
    end

    ImageOptimizer --> CDN[CDN / Cloud Storage];
    ImageOptimizer --> DB[(Main DB: Product Image URLs)];
    DB -- Webhook / SSE --> API;
    API -- Update --> LocalCache;
```

### Mobile UX Flow (375px First)
1. **Product Creation/Edit:** Maya is adding a new "Vegan Chocolate Cake." She taps "Add Photo" in the OHC mobile app.
2. **Capture:** She takes a photo of the cake on her kitchen counter using her phone's camera.
3. **Magic Wand Processing:** She taps a "Magic Enhance" button (styled with a subtle glassmorphic shimmer). A translucent loading overlay appears with a message: "AI is setting up the studio..."
4. **Review Options:** Within 5-10 seconds, the app presents 3-4 professional variations (e.g., pure white background, soft pastel background, marble countertop).
5. **Selection:** She taps her favorite. The app instantly updates the product listing and caches the WebP optimized image for fast loading.

### AI Agent Integration Points
- **Marketing & Advertising Agent ("The Promoter"):** Takes the optimized, AI-generated product photo and automatically creates an Instagram post and a promotional email draft featuring the new item.
- **Operations Agent ("The Manager"):** Links the generated images to the correct product variants (e.g., the red dress vs. the blue dress) in the catalog and inventory system.

### Key Design Decisions
- **Asynchronous Processing:** Image generation is slow. The mobile app uploads the raw image, creates a placeholder, and uses SSE (Server-Sent Events) or polling to update the UI when the AI finishes processing, preventing blocking.
- **Context-Aware Scene Generation:** The AI photography agent uses the product description and category (e.g., "Food", "Clothing") to determine the appropriate background prompt, eliminating the need for the user to write complex AI prompts.
- **Auto-Optimization:** All generated images are automatically compressed to WebP and resized for mobile, desktop, and thumbnail views before saving to the CDN, ensuring fast storefront load times.

## Implementation Prompt
Implement the Autonomous AI Product Photography Studio pipeline and UI.
- **User-Facing Outcome:** Users can upload a raw, unedited photo of a product from their phone and tap a button to automatically remove the background, enhance lighting, and generate professional studio-quality variations.
- **CUJ (Critical User Journey):**
  1. User creates or edits a product in the mobile app.
  2. User uploads a raw photo.
  3. User taps "Magic Enhance".
  4. App displays a loading state while the backend processes the image.
  5. User is presented with AI-generated variations.
  6. User selects a variation, and it is saved as the primary product image.
- **Acceptance Criteria:**
  - The UI for uploading and initiating the enhancement must work flawlessly on a 375px screen.
  - The backend successfully orchestrates background removal and AI scene generation (can be mocked with a delay and pre-defined images for testing purposes if necessary, but the pipeline must be real).
  - Images are compressed and stored securely.
  - The original and processed images are clearly linked in the data model.

## Priority
P1

## Estimated Scope
Medium
