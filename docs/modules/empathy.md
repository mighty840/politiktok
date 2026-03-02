# F09 -- Empathy Simulator

Simulates how different audience segments would perceive and react to political messaging, helping campaign teams refine their communication strategies before public release.

## Key Features

- **Persona modeling**: Define audience personas with demographics, concerns, values, and communication style preferences.
- **Multi-persona simulation**: Test messaging against multiple personas simultaneously.
- **Reaction analysis**: Each persona provides a simulated reaction with sentiment, key concerns, and a persuasion score.
- **Aggregate insights**: Summary view across all personas identifying which segments respond positively and which need different messaging.
- **Iterative refinement**: Revise messaging and re-simulate to track improvement.
- **Pre-built personas**: Library of common voter archetypes ready for immediate use.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `simulate_reactions` | `empathy/simulate` | Simulate persona reactions to a policy text |
| `list_personas` | `empathy/personas` | List available personas |
| `create_persona` | `empathy/create-persona` | Create a custom persona |
| `get_simulation` | `empathy/get-simulation` | Retrieve a past simulation result |
| `list_simulations` | `empathy/list-simulations` | List past simulation results |

## Persona Structure

Each persona is defined by:

- **Name**: A descriptive label (e.g., "Suburban Parent")
- **Demographic**: Age range, location type, income bracket
- **Concerns**: Top issues they care about (education, taxes, safety)
- **Values**: Core value drivers (family, independence, community)
- **Communication style**: Preferred tone (formal, conversational, data-driven)

## Reaction Output

For each persona, the simulation returns:

- **Reaction**: A paragraph describing how this person would likely respond
- **Sentiment**: positive, negative, neutral, or mixed
- **Key concerns**: Specific concerns the messaging raises or addresses
- **Persuasion score**: 0.0 to 1.0 indicating how persuasive the message would be

## UI Components

- **Empathy simulator** (`/empathy`): Text input for policy messaging with persona selection grid.
- **Reaction cards**: Visual display of each persona's reaction with sentiment indicators and persuasion meters.
- **Aggregate summary**: Cross-persona overview highlighting strongest and weakest audience segments.
- **Persona manager**: Create and edit custom personas.

## Database Tables

- `personas` -- persona definitions with demographics, concerns, values
- `empathy_simulations` -- simulation results with reactions per persona (jsonb)
