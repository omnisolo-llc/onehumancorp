<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# User Guide: OHC Slint App

## 1. Overview

This guide covers the Bazel-native Slint app workflow in `src/app`.
The app provides a unified, mobile-first onboarding experience for small business owners.

## 2. Onboarding Flow

The OHC Slint app features a comprehensive 11-step onboarding wizard:
1.  **Welcome**: Introduction to OneHumanCorp. (Step 0)
2.  **Business Type**: Selection of business category. (Step 1)
3.  **Name & Description**: Naming the business and AI-assisted description. (Step 2)
4.  **Sell Categories**: Multi-select for what the business sells. (Step 3)
5.  **Payments**: Choice of payment collection. (Step 4)
6.  **Admin Account**: Creating the administrator account. (Step 5)
7.  **Template Selection**: Choosing a website design theme. (Step 6)
8.  **Product Add**: Adding the first product or service. (Step 7)
9.  **Domain Choice**: Choosing between a free OHC subdomain or a custom domain. (Step 8)
10. **Review & Launch**: Summary of setup and triggering the AI deployment. (Step 9)
11. **Checklist**: Post-launch checklist for the business owner. (Step 10)
12. **Instant Build**: AI-powered one-step setup option. (Step 11)

## 3. Running Tests

Run the following from the repository root:

```bash
bazelisk test //src/app:app_test
```

## 4. Documentation

Please refer to the detailed architecture documents in the `docs/` folder:
- [KAIROS Orchestration Design Phase 4](./business/features/kairos_orchestration_phase4/design-doc.md)

</div>

<div markdown="1" style="font-family: Outfit, Inter, sans-serif; padding: 20px; font-size: 12px; color: #888;">
Last synced: 2026-04-30 17:55:00
</div>
