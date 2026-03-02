# Introduction

**PolitikTok** is an AI-powered political campaign operations platform built with Rust, Dioxus, and local LLMs. It provides 26 integrated modules that help political parties manage everything from volunteer coordination and policy communication to opposition research and electoral compliance.

## Why PolitikTok?

Political campaigns generate and consume massive amounts of data — voter information, social media sentiment, policy documents, volunteer availability, donor records, and more. Traditional tools are fragmented, expensive, and often require sending sensitive data to third-party cloud providers.

PolitikTok brings all of these capabilities into a single, self-hosted platform powered by local AI. Your data never leaves your infrastructure.

## Key Capabilities

- **Volunteer Management** — AI-powered matching, churn prediction, and automated outreach
- **Citizen Engagement** — RAG-based policy chatbot, multilingual content, canvassing scripts
- **Intelligence** — Sentiment monitoring, opposition research, media tracking, disinformation alerts
- **Content Generation** — Campaign copy, briefings, meeting summaries, translated materials
- **Analysis** — Coalition tension detection, faction mapping, empathy simulation
- **Compliance** — Electoral compliance reporting, audit trails, data governance
- **Administration** — User management, module configuration, LLM settings, health monitoring

## Architecture

PolitikTok is a fullstack Rust application using Dioxus 0.7.3 for both server-side rendering and client-side hydration. The backend is powered by Axum, with PostgreSQL for relational data, Qdrant for vector search (RAG), and Keycloak for authentication.

All AI features use an OpenAI-compatible API interface, which works with Ollama, vLLM, or any compatible provider.

## Getting Started

Head to the [Installation](./getting-started/installation.md) guide to set up your development environment.
