# F18 -- Policy Diff

Compares two policy documents side-by-side, identifying changes in language, intent, and impact using LLM-powered analysis.

## Key Features

- **Side-by-side comparison**: Submit two policy documents for detailed comparison.
- **Change detection**: Identify additions, removals, modifications, and rewording across sections.
- **Significance rating**: Each change is rated for its policy significance.
- **Intent analysis**: Detect shifts in policy intent beyond surface-level wording changes.
- **Impact summary**: Overall summary of how the changes affect policy direction.
- **Section-level granularity**: Changes are attributed to specific document sections.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `compare_policies` | `policy-diff/compare` | Compare two policy documents |
| `list_diffs` | `policy-diff/list` | List past policy comparisons |
| `get_diff` | `policy-diff/get` | Retrieve a specific comparison |

## Change Structure

Each detected change includes:

- **Section**: The document section where the change occurs
- **Change type**: added, removed, modified, or reworded
- **Old text**: The original text (empty for additions)
- **New text**: The updated text (empty for removals)
- **Significance**: minor, moderate, major, or critical

## Diff Output

A complete policy diff includes:

- **Document A title**: Name of the first document
- **Document B title**: Name of the second document
- **Changes**: Ordered list of detected changes
- **Summary**: Narrative overview of the most important changes

## UI Components

- **Policy diff page** (`/policy-diff`): Two text input panels for document A and document B with diff results below.
- **Change list**: Color-coded change entries (green for additions, red for removals, yellow for modifications).
- **Significance filter**: Filter changes by significance level.
- **Summary panel**: Overall impact narrative.

## Database Tables

- `policy_diffs` -- document titles, changes (jsonb), summary, timestamps
