# F05 -- Opposition Research & Debate Briefing

Compiles and analyzes opposition records, statements, and positions to generate comprehensive debate preparation briefings and detect contradictions in public statements.

## Key Features

- **Opponent profiles**: Maintain detailed dossiers on political opponents with policy positions, voting records, and public statements.
- **Debate briefing generation**: LLM-powered structured briefings covering background, vulnerabilities, policy weaknesses, attack lines, and defense preparation.
- **Contradiction detection**: Automated identification of flip-flops and inconsistencies in opponents' stated positions.
- **Position tracking**: Record and organize opponents' positions on key issues over time.
- **Evidence linking**: Attach sources and evidence to specific claims and contradictions.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `create_opponent` | `opposition/create` | Create an opponent profile |
| `update_opponent` | `opposition/update` | Update opponent details |
| `list_opponents` | `opposition/list` | List all tracked opponents |
| `get_opponent` | `opposition/get` | Fetch full opponent profile |
| `generate_briefing` | `opposition/briefing` | Generate a debate briefing |
| `detect_contradictions` | `opposition/contradictions` | Find contradictions in positions |

## Briefing Structure

Generated briefings follow a standardized markdown structure:

1. **Background** -- biographical and political context
2. **Key Vulnerabilities** -- areas where the opponent is weakest
3. **Policy Weaknesses** -- specific policy positions open to challenge
4. **Recommended Attack Lines** -- suggested debate points
5. **Defense Preparation** -- anticipated attacks and prepared responses

## UI Components

- **Opponents list** (`/opposition`): Table of tracked opponents with quick-access to briefings.
- **Opponent detail** (`/opposition/:id`): Full profile with positions, contradictions, and generated briefings.
- **Briefing viewer**: Rendered markdown briefing with section navigation.
- **Contradiction timeline**: Visual display of detected inconsistencies.

## Database Tables

- `opponents` -- opponent profiles with positions (jsonb), voting records
- `contradictions` -- detected inconsistencies with evidence and confidence scores
