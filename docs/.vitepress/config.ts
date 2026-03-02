import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'PolitikTok',
  description: 'AI-powered campaign intelligence platform — documentation',
  base: '/politiktok/',

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/politiktok/favicon.svg' }],
  ],

  themeConfig: {
    siteTitle: 'PolitikTok',
    logo: '/logo.svg',

    nav: [
      { text: 'Guide', link: '/guide/introduction' },
      { text: 'Self-Hosting', link: '/self-hosting/overview' },
      {
        text: 'More',
        items: [
          { text: 'Configuration', link: '/configuration/environment-variables' },
          { text: 'Architecture', link: '/architecture/overview' },
          { text: 'API', link: '/api/overview' },
          { text: 'Modules', link: '/modules/volunteer-matching' },
          { text: 'Deployment', link: '/deployment/docker' },
          { text: 'Contributing', link: '/contributing/development-setup' },
        ],
      },
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Introduction', link: '/guide/introduction' },
          { text: 'Requirements', link: '/guide/requirements' },
          { text: 'Install with Docker', link: '/guide/docker' },
          { text: 'Install Manually', link: '/guide/installation' },
          { text: 'Quick Start', link: '/guide/quick-start' },
        ],
      },
      {
        text: 'Self-Hosting',
        items: [
          { text: 'Overview', link: '/self-hosting/overview' },
          { text: 'Server Setup', link: '/self-hosting/server-setup' },
          { text: 'Docker Compose Production', link: '/self-hosting/docker-compose' },
          { text: 'Domain & DNS', link: '/self-hosting/domain-dns' },
          { text: 'Reverse Proxy & SSL', link: '/self-hosting/reverse-proxy-ssl' },
          { text: 'Backups', link: '/self-hosting/backups' },
          { text: 'Maintenance', link: '/self-hosting/maintenance' },
        ],
      },
      {
        text: 'Configuration',
        items: [
          { text: 'Environment Variables', link: '/configuration/environment-variables' },
          { text: 'Database', link: '/configuration/database' },
          { text: 'Authentication', link: '/configuration/authentication' },
          { text: 'LLM Integration', link: '/configuration/llm-integration' },
          { text: 'Theming', link: '/configuration/theming' },
          { text: 'Internationalization', link: '/configuration/i18n' },
          { text: 'CORS & Security', link: '/configuration/cors-security' },
        ],
      },
      {
        text: 'Architecture',
        items: [
          { text: 'Overview', link: '/architecture/overview' },
          { text: 'Tech Stack', link: '/architecture/tech-stack' },
          { text: 'Project Structure', link: '/architecture/project-structure' },
          { text: 'RAG Pipeline', link: '/architecture/rag-pipeline' },
        ],
      },
      {
        text: 'API',
        items: [
          { text: 'Overview', link: '/api/overview' },
        ],
      },
      {
        text: 'Modules',
        collapsed: true,
        items: [
          { text: 'F01 — Volunteer Matching', link: '/modules/volunteer-matching' },
          { text: 'F02 — Policy Chatbot', link: '/modules/policy-chatbot' },
          { text: 'F03 — Sentiment Monitor', link: '/modules/sentiment-monitor' },
          { text: 'F04 — Campaign Copy', link: '/modules/campaign-copy' },
          { text: 'F05 — Opposition Research', link: '/modules/opposition-research' },
          { text: 'F06 — Canvassing Scripts', link: '/modules/canvassing' },
          { text: 'F07 — Fundraising Assistant', link: '/modules/fundraising' },
          { text: 'F08 — Accountability Engine', link: '/modules/accountability' },
          { text: 'F09 — Empathy Simulator', link: '/modules/empathy' },
          { text: 'F10 — Narrative Contagion', link: '/modules/narrative' },
          { text: 'F11 — Coalition Detector', link: '/modules/coalition' },
          { text: 'F12 — Candidate Briefings', link: '/modules/briefings' },
          { text: 'F13 — Call Intelligence', link: '/modules/call-intelligence' },
          { text: 'F14 — Coaching & Debate', link: '/modules/coaching' },
          { text: 'F15 — Multilingual Outreach', link: '/modules/multilingual' },
          { text: 'F16 — Question Anticipation', link: '/modules/question-anticipation' },
          { text: 'F17 — Local Issues', link: '/modules/local-issues' },
          { text: 'F18 — Policy Diff', link: '/modules/policy-diff' },
          { text: 'F19 — Faction Mapper', link: '/modules/faction-mapper' },
          { text: 'F20 — Regulatory Monitor', link: '/modules/regulatory' },
          { text: 'F21 — Media Monitor', link: '/modules/media-monitor' },
          { text: 'F22 — Disinfo Warning', link: '/modules/disinfo' },
          { text: 'F23 — Compliance', link: '/modules/compliance' },
          { text: 'F24 — Meeting Summarizer', link: '/modules/meetings' },
          { text: 'F25 — Knowledge Base', link: '/modules/knowledge-base' },
          { text: 'F26 — Admin Panel', link: '/modules/admin' },
        ],
      },
      {
        text: 'Deployment',
        items: [
          { text: 'Docker', link: '/deployment/docker' },
          { text: 'Manual', link: '/deployment/manual' },
          { text: 'CI/CD', link: '/deployment/ci-cd' },
          { text: 'Scaling', link: '/deployment/scaling' },
        ],
      },
      {
        text: 'Contributing',
        items: [
          { text: 'Development Setup', link: '/contributing/development-setup' },
          { text: 'Testing', link: '/contributing/testing' },
          { text: 'Code Style', link: '/contributing/code-style' },
          { text: 'Pull Requests', link: '/contributing/pull-requests' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/mighty840/politiktok' },
    ],

    search: {
      provider: 'local',
    },
  },
})
