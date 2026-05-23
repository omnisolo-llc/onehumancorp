<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# User Guide: OHC App

## 1. Overview

This guide covers the Flutter app workflow in `src/app`.
The app provides a unified, mobile-first onboarding experience for small business owners.

## 2. Onboarding Flow

The OHC App features a comprehensive 12-step onboarding wizard:
1.  **Welcome**: Introduction to OneHumanCorp.
2.  **Business Type**: Selection of business category (Online Store, Service, etc.).
3.  **Name & Description**: Naming the business and AI-assisted description.
4.  **Sell Categories**: Multi-select for what the business sells (Physical, Digital, Services, etc.).
5.  **Payments**: Choice of payment collection (Online, In-person, Both).
6.  **Admin Account**: Creating the administrator account.
7.  **Template Selection**: Choosing a website design theme (Modern, Classic, Bold).
8.  **Product Add**: Adding the first product or service with price and photo.
9.  **Domain Choice**: Choosing between a free OHC subdomain or a custom domain.
10. **Review & Launch**: Summary of setup and triggering the AI deployment.
11. **Checklist**: Post-launch checklist for the business owner.
12. **Deployment**: AI-powered storefront deployment.

## 3. Running Tests

Run the following from the repository root:

```bash
bazelisk test //src/app:app_test
```

## 4. Documentation

Please refer to the detailed architecture documents in the `docs/` folder:
- [KAIROS Architecture](../technical/architecture/kairos/master-design-doc.md)
- [API Playbook](../api/playbook.md)

</div>

<div markdown="1" style="font-family: Outfit, Inter, sans-serif; padding: 20px; font-size: 12px; color: #888;">
Last synced: 2026-04-30 17:55:00
</div>