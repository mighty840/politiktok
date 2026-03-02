# F15 -- Multilingual Outreach

Automates translation and cultural adaptation of campaign materials for multilingual communities while preserving tone, intent, and political nuance.

## Key Features

- **Translation with adaptation**: Not just word-for-word translation -- culturally adapts content for target audiences.
- **Cultural notes**: Provides context notes explaining cultural considerations for each translation.
- **Tone preservation**: Maintains the original message's tone and persuasive intent across languages.
- **Multiple target languages**: Translate to any supported language in a single operation.
- **Translation history**: Store and retrieve past translations for consistency.
- **Language directory**: Browsable list of supported languages.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `translate` | `multilingual/translate` | Translate and culturally adapt text |
| `list_translations` | `multilingual/list` | List past translations |
| `get_translation` | `multilingual/get` | Retrieve a specific translation |
| `list_languages` | `multilingual/languages` | List supported languages |

## Translation Output

Each translation includes:

- **Source text**: The original content
- **Source language**: Detected or specified source language
- **Target language**: The language translated into
- **Translated text**: The adapted translation
- **Cultural notes**: List of adaptation notes (e.g., "Formal address form used as customary in political contexts", "Healthcare terminology adapted for local system")

## UI Components

- **Translation page** (`/multilingual`): Source text input with language selectors and translation output.
- **Side-by-side view**: Original and translated text displayed in parallel columns.
- **Cultural notes panel**: Expandable notes explaining adaptation decisions.
- **Translation history**: Browse past translations with search and language filtering.

## Database Tables

- `translations` -- source text, source/target languages, translated text, cultural notes (text[])
