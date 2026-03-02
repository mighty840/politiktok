# Theming

PolitikTok supports dark and light themes with a modern glassmorphism design system.

## Theme Switching

Users can toggle between dark and light themes using the toggle in the sidebar footer. The preference is persisted in `localStorage` under the key `politiktok-theme`.

## Available Themes

### Dark Theme (`politiktok-dark`)

The default theme with deep navy backgrounds and indigo accents:

- **Background:** `#0b0f1a` (deep navy)
- **Surface:** `rgba(30, 41, 59, 0.5)` (glass effect)
- **Accent:** `#6366f1` (indigo)
- **CTA:** `#f43f5e` (rose)
- **Text:** `#e2e8f0` (light slate)

### Light Theme (`politiktok-light`)

An inverted theme with white backgrounds:

- **Background:** `#f8fafc` (off-white)
- **Surface:** `rgba(255, 255, 255, 0.6)` (glass effect)
- **Accent:** `#4f46e5` (deeper indigo)
- **CTA:** `#e11d48` (deeper rose)
- **Text:** `#1e293b` (dark slate)

## CSS Custom Properties

All theme colors are defined as CSS custom properties in `assets/main.css`. You can customize them by overriding the properties under the `[data-theme]` selectors:

```css
[data-theme="politiktok-dark"] {
    --color-bg-primary:    #0b0f1a;
    --color-bg-secondary:  #111827;
    --color-bg-tertiary:   #1e293b;
    --color-accent:        #6366f1;
    --color-rose:          #f43f5e;
    --glass-bg:            rgba(30, 41, 59, 0.5);
    --glass-border:        rgba(148, 163, 184, 0.12);
    --gradient-primary:    linear-gradient(135deg, #6366f1, #8b5cf6, #7c3aed);
    /* ... */
}
```

## Design System Components

### Glass Cards

The `.glass-card` class provides a translucent card with backdrop blur:

```css
.glass-card {
    background: var(--glass-bg);
    backdrop-filter: blur(12px);
    border: 1px solid var(--glass-border);
    border-radius: 1rem;
}
```

### Gradient Borders

The `.gradient-border` class adds a gradient border effect on hover using a pseudo-element.

### Animations

Built-in animation classes:

| Class | Effect |
|-------|--------|
| `.animate-fade-in` | Fade in (0.5s) |
| `.animate-slide-up` | Slide up + fade in |
| `.animate-slide-in-right` | Slide in from right |
| `.animate-scale-in` | Scale up + fade in |
| `.stagger-enter` | Cascading entrance for child elements |
| `.skeleton-shimmer` | Loading placeholder shimmer |
| `.btn-scale` | Button hover/active scale feedback |

## Creating a Custom Theme

To add a new theme:

1. Add a new `[data-theme="your-theme"]` block in `assets/main.css`
2. Define all required CSS custom properties
3. Update the theme toggle logic in `src/components/sidebar.rs`

## Typography

PolitikTok uses [Inter](https://fonts.google.com/specimen/Inter) (weights 300-900) loaded from Google Fonts. The font stack falls back to `system-ui, -apple-system, sans-serif`.
