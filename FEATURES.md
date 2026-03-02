# PolitikTok — Feature List

Features that are not yet implemented or are partially implemented.

## Platform Features

### Authentication & Authorization
- [ ] Social login (Google, GitHub OAuth)
- [ ] Multi-factor authentication (MFA/2FA)
- [ ] API key management for external integrations
- [ ] Fine-grained permission system per module
- [ ] Team/organization management with invitations

### Internationalization (i18n)
- [ ] UI language switching (locale selector)
- [ ] Translation files (JSON-based per locale)
- [ ] Right-to-left (RTL) layout support
- [ ] Locale-aware date/time formatting
- [ ] Locale-aware number/currency formatting
- [ ] Pluralization rules per locale

### Theming & Customization
- [ ] Custom theme creation UI (color picker)
- [ ] Per-organization branding (logo, colors)
- [ ] High-contrast accessibility theme
- [ ] Font size preferences
- [ ] Compact/comfortable density toggle

### Notifications & Alerts
- [ ] Email notifications for alerts and events
- [ ] SMS notifications (via Twilio or similar)
- [ ] In-app notification center
- [ ] Webhook integrations for external alerting
- [ ] Push notifications (web push API)
- [ ] Notification preferences per user

### Data & Export
- [ ] CSV/Excel export for all data tables
- [ ] PDF report generation
- [ ] Scheduled automated reports
- [ ] Data import from external sources
- [ ] Bulk operations on records

### Search & Discovery
- [ ] Global search across all modules
- [ ] Saved searches / filters
- [ ] Recent items history
- [ ] Bookmarks / favorites

### Collaboration
- [ ] Comments / notes on any record
- [ ] Activity feed per module
- [ ] Shared dashboards
- [ ] @mentions in comments
- [ ] Real-time collaboration indicators

## Module-Specific Features

### Volunteer Matching (F01)
- [ ] Volunteer churn prediction model (ML)
- [ ] Automated volunteer outreach scheduling
- [ ] Availability calendar integration
- [ ] Skill verification system

### Policy Chatbot (F02)
- [ ] Document upload UI (PDF, DOCX ingestion)
- [ ] Citation highlighting in responses
- [ ] Conversation sharing
- [ ] Suggested follow-up questions

### Sentiment Monitor (F03)
- [ ] Live social media API integration (Twitter/X, Reddit)
- [ ] Automated spike alerting
- [ ] Sentiment trend dashboards with date range filtering
- [ ] Geographic sentiment heatmap

### Campaign Copy (F04)
- [ ] A/B test variant generation
- [ ] Brand voice consistency checker
- [ ] Image/social media card generation
- [ ] Content calendar scheduling

### Opposition Research (F05)
- [ ] Automated web scraping for opponent updates
- [ ] Timeline of opponent positions
- [ ] Side-by-side comparison views
- [ ] Automated daily briefing emails

### Canvassing (F06)
- [ ] Map-based territory assignment
- [ ] Mobile-optimized canvassing interface
- [ ] Door-knock tracking and analytics
- [ ] Route optimization

### Fundraising (F07)
- [ ] Payment processing integration (Stripe)
- [ ] Donation page builder
- [ ] Donor segmentation analytics
- [ ] Recurring donation management
- [ ] FEC/compliance auto-filing

### Accountability (F08)
- [ ] Public-facing accountability dashboard
- [ ] Automated evidence collection from news feeds
- [ ] Promise timeline visualization

### Coaching & Debate (F14)
- [ ] Video recording and playback
- [ ] AI-scored debate performance metrics
- [ ] Timed response practice mode

### Media Monitor (F21)
- [ ] RSS/news feed auto-ingestion
- [ ] Media sentiment trends over time
- [ ] Journalist contact management
- [ ] Press release drafting

### Compliance (F23)
- [ ] FEC filing format export
- [ ] Automated compliance deadline tracking
- [ ] Expenditure categorization
- [ ] Audit trail with tamper detection

### Admin Panel (F26)
- [ ] User activity analytics dashboard
- [ ] Module usage metrics and billing
- [ ] System resource monitoring graphs
- [ ] Automated health check alerts
- [ ] Configuration backup and restore

## Infrastructure Features

### Performance
- [ ] LLM response caching
- [ ] Database query result caching (Redis)
- [ ] Static asset CDN support
- [ ] Connection pooling optimization (PgBouncer)

### Observability
- [ ] Prometheus metrics endpoint
- [ ] OpenTelemetry tracing
- [ ] Grafana dashboard templates
- [ ] Error tracking (Sentry integration)

### DevOps
- [ ] Dockerfile for production builds
- [ ] Kubernetes Helm chart
- [ ] Terraform/Pulumi infrastructure templates
- [ ] Automated database migration on startup
- [ ] Blue/green deployment support

### API
- [ ] REST API documentation (OpenAPI/Swagger)
- [ ] GraphQL API layer
- [ ] Rate limiting per user/API key
- [ ] API versioning
- [ ] Webhook event system
