# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ambassador-reply.spec.ts >> Ambassador Auto-Responder CUJ >> Owner connects Meta Graph API and approves Ambassador drafted reply
- Location: src/e2e/ambassador-reply.spec.ts:4:7

# Error details

```
Error: apiRequestContext.post: connect ECONNREFUSED ::1:8080
Call log:
  - → POST http://localhost:8080/api/v1/webhooks/meta
    - user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.7778.96 Safari/537.36
    - accept: */*
    - accept-encoding: gzip,deflate,br
    - content-type: application/json
    - content-length: 169

```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e2]:
    - generic [ref=e5]:
      - generic [ref=e7]: Premium
      - heading "Tool Integrations" [level=1] [ref=e8]
      - paragraph [ref=e9]: Supercharge your workflow by connecting your favorite tools.
    - main [ref=e10]:
      - generic [ref=e11]:
        - button "All" [pressed] [ref=e12]
        - button "Marketing" [ref=e13]
        - button "Operations" [ref=e14]
        - button "Finance" [ref=e15]
        - button "Social" [ref=e16]
      - status [ref=e17]: Meta Graph API connected.
      - generic [ref=e18]:
        - generic [ref=e19]:
          - generic [ref=e20]:
            - generic [ref=e21]: 📱
            - generic [ref=e22]: disconnected
          - heading "Ayrshare" [level=3] [ref=e23]
          - paragraph [ref=e24]: Single API for posting and retrieving messages across social networks.
          - button "Connect" [ref=e25]
        - generic [ref=e26]:
          - generic [ref=e27]:
            - generic [ref=e28]: 📅
            - generic [ref=e29]: disconnected
          - heading "Cal.com" [level=3] [ref=e30]
          - paragraph [ref=e31]: Zero-Config Booking & Calendar Sync.
          - button "Connect" [ref=e32]
        - generic [ref=e33]:
          - generic [ref=e34]:
            - generic [ref=e35]: 📨
            - generic [ref=e36]: disconnected
          - heading "MailerLite" [level=3] [ref=e37]
          - paragraph [ref=e38]: Embedded, No-Jargon Email Campaigns.
          - button "Connect" [ref=e39]
        - generic [ref=e40]:
          - generic [ref=e41]:
            - generic [ref=e42]: 🌎
            - generic [ref=e43]: disconnected
          - heading "Mercado Pago" [level=3] [ref=e44]
          - paragraph [ref=e45]: Accept credit cards and local payment methods in Latin America.
          - button "Connect" [ref=e46]
        - generic [ref=e47]:
          - generic [ref=e48]:
            - generic [ref=e49]: 📦
            - generic [ref=e50]: disconnected
          - heading "Shippo" [level=3] [ref=e51]
          - paragraph [ref=e52]: Painless Shipping Labels & Tracking.
          - button "Connect" [ref=e53]
        - generic [ref=e54]:
          - generic [ref=e55]:
            - generic [ref=e56]: 🔔
            - generic [ref=e57]: disconnected
          - heading "Twilio Conversations" [level=3] [ref=e58]
          - paragraph [ref=e59]: Central omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat.
          - button "Connect" [ref=e60]
        - generic [ref=e61]:
          - generic [ref=e62]:
            - generic [ref=e63]: 📹
            - generic [ref=e64]: disconnected
          - heading "Whereby" [level=3] [ref=e65]
          - paragraph [ref=e66]: Zero-Setup Online Lessons and video conferencing.
          - button "Connect" [ref=e67]
        - generic [ref=e68]:
          - generic [ref=e69]:
            - generic [ref=e70]: 📧
            - generic [ref=e71]: disconnected
          - heading "Resend" [level=3] [ref=e72]
          - paragraph [ref=e73]: Transactional and Marketing Emails.
          - button "Connect" [ref=e74]
        - generic [ref=e75]:
          - generic [ref=e76]:
            - generic [ref=e77]: 💬
            - generic [ref=e78]: disconnected
          - heading "WhatsApp Cloud API" [level=3] [ref=e79]
          - paragraph [ref=e80]: Central WhatsApp Inbox for Work Triage and Customer Assistant.
          - button "Connect" [ref=e81]
        - generic [ref=e82]:
          - generic [ref=e83]:
            - generic [ref=e84]: 💬
            - generic [ref=e85]: connected
          - heading "Meta Graph API" [level=3] [ref=e86]
          - paragraph [ref=e87]: Central Instagram and Facebook Inbox.
          - button "Manage" [active] [ref=e88]
        - generic [ref=e89]:
          - generic [ref=e90]:
            - generic [ref=e91]: 📥
            - generic [ref=e92]: disconnected
          - heading "Front" [level=3] [ref=e93]
          - paragraph [ref=e94]: Central omnichannel inbox aggregating messages across all channels.
          - button "Connect" [ref=e95]
        - generic [ref=e96]:
          - generic [ref=e97]:
            - generic [ref=e98]: 📹
            - generic [ref=e99]: disconnected
          - heading "Zoom" [level=3] [ref=e100]
          - paragraph [ref=e101]: Automated Online Lesson Links.
          - button "Connect" [ref=e102]
  - button "Help" [ref=e105]:
    - img [ref=e106]
  - button "Open help chat" [ref=e109]:
    - generic [ref=e110]: ✨
    - generic [ref=e111]: Ask anything
  - button "Open Next.js Dev Tools" [ref=e117] [cursor=pointer]:
    - img [ref=e118]
  - alert [ref=e121]
```