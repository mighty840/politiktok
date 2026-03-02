import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'PolitikTok',
  description: 'AI-powered campaign intelligence platform',

  themeConfig: {
    siteTitle: 'PolitikTok',

    nav: [
      { text: 'Getting Started', link: '/getting-started/installation' },
      { text: 'Architecture', link: '/architecture/overview' },
      { text: 'Modules', link: '/modules/volunteer-matching' },
      { text: 'Development', link: '/development/contributing' },
    ],

    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Introduction', link: '/introduction' },
        ],
      },
      {
        text: 'Getting Started',
        items: [
          { text: 'Installation', link: '/getting-started/installation' },
          { text: 'Configuration', link: '/getting-started/configuration' },
          { text: 'Docker Setup', link: '/getting-started/docker' },
        ],
      },
      {
        text: 'Architecture',
        items: [
          { text: 'Overview', link: '/architecture/overview' },
          { text: 'Tech Stack', link: '/architecture/tech-stack' },
          { text: 'Project Structure', link: '/architecture/project-structure' },
          { text: 'Authentication', link: '/architecture/authentication' },
          { text: 'LLM Integration', link: '/architecture/llm-integration' },
          { text: 'RAG Pipeline', link: '/architecture/rag-pipeline' },
        ],
      },
      {
        text: 'Modules',
        collapsed: false,
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
        text: 'Development',
        items: [
          { text: 'Contributing', link: '/development/contributing' },
          { text: 'CI/CD', link: '/development/ci-cd' },
          { text: 'Testing', link: '/development/testing' },
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
