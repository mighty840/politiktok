# API Overview

PolitikTok exposes server functions as HTTP endpoints via Dioxus fullstack.

## Architecture

PolitikTok uses Dioxus server functions rather than a traditional REST API. Each server function is:

- Defined with `#[server(endpoint = "name")]`
- Automatically serialized/deserialized
- Callable from both client-side components and external HTTP requests

## Authentication

All API endpoints (except `check-auth`) require an authenticated session. Authentication is handled via Keycloak OIDC session cookies.

## Endpoint Conventions

Server function endpoints follow the pattern:

```
POST /api/{endpoint-name}
Content-Type: application/cbor
```

## Module Endpoints

Each module exposes its own set of server functions. See the individual module documentation for endpoint details:

| Module | Key Endpoints |
|--------|--------------|
| [Volunteer Matching](/modules/volunteer-matching) | `list-volunteers`, `match-volunteer`, `volunteer-detail` |
| [Policy Chatbot](/modules/policy-chatbot) | `policy-chat`, `policy-ingest`, `list-policy-sessions` |
| [Sentiment Monitor](/modules/sentiment-monitor) | `ingest-posts`, `sentiment-summary`, `detect-spikes` |
| [Campaign Copy](/modules/campaign-copy) | `generate-copy`, `list-copy-history` |
| [Opposition Research](/modules/opposition-research) | `list-opponents`, `generate-briefing`, `detect-contradictions` |
| [Canvassing](/modules/canvassing) | `generate-script` |
| [Fundraising](/modules/fundraising) | `list-donors`, `record-donation`, `draft-solicitation` |
| [Accountability](/modules/accountability) | `extract-commitments`, `add-evidence`, `accountability-report` |
| [Empathy Simulator](/modules/empathy) | `simulate-reaction` |
| [Narrative Contagion](/modules/narrative) | `extract-narratives`, `predict-virality` |
| [Coalition Detector](/modules/coalition) | `detect-tensions` |
| [Candidate Briefings](/modules/briefings) | `generate-briefing` |
| [Call Intelligence](/modules/call-intelligence) | `analyze-transcript`, `call-trends` |
| [Coaching & Debate](/modules/coaching) | `coaching-exchange` |
| [Multilingual](/modules/multilingual) | `translate-content` |
| [Question Anticipation](/modules/question-anticipation) | `anticipate-questions`, `preparation-checklist` |
| [Local Issues](/modules/local-issues) | `analyze-local-issues` |
| [Policy Diff](/modules/policy-diff) | `compare-policies` |
| [Faction Mapper](/modules/faction-mapper) | `map-factions`, `consensus-finder` |
| [Regulatory Monitor](/modules/regulatory) | `analyze-regulation` |
| [Media Monitor](/modules/media-monitor) | `analyze-article`, `compare-coverage` |
| [Disinfo Warning](/modules/disinfo) | `analyze-disinfo`, `counter-messaging` |
| [Compliance](/modules/compliance) | `generate-report`, `compliance-check` |
| [Meetings](/modules/meetings) | `summarize-meeting` |
| [Knowledge Base](/modules/knowledge-base) | `kb-query`, `kb-ingest` |
| [Admin](/modules/admin) | `admin-health`, `admin-list-users` |

## Feedback Endpoint

All modules share a common feedback endpoint:

```
POST /api/record-feedback
```

Parameters: `module_id`, `resource_id`, `is_positive`, `is_active`
