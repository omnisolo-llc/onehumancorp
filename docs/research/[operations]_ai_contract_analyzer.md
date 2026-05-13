# AI Contract Analyzer

## Problem Statement
Freelancers, independent consultants, and specialized service providers are frequently asked to sign highly complex client Master Services Agreements (MSAs) or Non-Disclosure Agreements (NDAs). They simply cannot afford to hire a lawyer to review a dense 10-page legal document for a $2000 project.

## Research Report
Legal anxiety represents a major barrier to growth for independent service providers. They often sign intimidating documents blindly to secure the work, thereby exposing themselves to massive hidden risks (e.g., predatory, multi-year non-compete clauses).

## Design Doc
### Architecture Vision
- **Entities**: UploadedLegalDocument, RiskFlag, PlainLanguageSummary.
- **UX Flow**:
  1. The user securely uploads a PDF contract received from a prospective client.
  2. The system analyzes the text and explicitly highlights unusual or predatory clauses (e.g., 'Warning: This non-compete clause lasts for 5 years. The industry standard is 1 year.').
  3. The system provides a highly simplified, plain-English summary of their core obligations.
- **Mobile UX**: A simple, intuitive document upload interface paired with a clear red/yellow/green risk assessment report.
- **Agent Integration**: The Legal Agent utilizes a specialized LLM, fine-tuned specifically on standard SMB contracts, to reliably flag anomalies and summarize legalese.

## Implementation Prompt
**Outcome**: Develop a specialized tool that reads uploaded legal contracts and provides a highly accessible, plain-language summary of core risks and obligations.
**Critical User Journey**:
1. The user receives a complex contract from a client.
2. The user uploads the document securely to the OHC platform.
3. The user reads the AI-generated summary and utilizes that insight to decide whether to sign or negotiate.
**Acceptance Criteria**: The feature must include a strict, unmissable legal disclaimer stating explicitly that the automated output does not constitute professional legal advice. The AI must accurately identify historically problematic clauses (e.g., perpetual IP assignment, extreme liability indemnification).

## Priority
P2

## Estimated Scope
Large
