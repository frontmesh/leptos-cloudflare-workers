# Using leptos-cloudflare-workers as a Template

This guide explains how to use this project as a starting point for your own Leptos + Cloudflare Workers application.

## Quick Start (Recommended)

### Step 1: Use GitHub Template

The easiest way to get started:

1. Visit: https://github.com/frontmesh/leptos-cloudflare-workers
2. Click the green **"Use this template"** button
3. Click **"Create a new repository"**
4. Choose your repository name and settings
5. Click **"Create repository from template"**

### Step 2: Clone Your New Repository

```bash
git clone git@github.com:YOUR_USERNAME/YOUR_REPO_NAME.git
cd YOUR_REPO_NAME
```

### Step 3: Run the Setup Script

```bash
bash scripts/setup.sh
```

This will:
- Prompt you for your project name
- Update `Cargo.toml` with your project name
- Update `wrangler.toml` for Cloudflare Workers
- Update `README.md` with the correct references

You can also pass the project name as an argument:

```bash
bash scripts/setup.sh my-awesome-app
```

### Step 4: Verify the Setup

```bash
cargo leptos build
```

If the build succeeds, you're ready to start developing!

## Manual Setup (Alternative)

If you prefer to clone and configure manually:

### 1. Clone the Repository

```bash
git clone git@github.com:frontmesh/leptos-cloudflare-workers.git my-new-project
cd my-new-project
```

### 2. Update Project Name in Configuration Files

**Cargo.toml:**
```toml
[package]
name = "my-new-project"  # Change this
version = "0.1.0"
edition = "2021"

# ... later in the file ...
[package.metadata.leptos]
output-name = "my-new-project"  # And this
```

**wrangler.toml:**
```toml
name = "my-new-project"  # Change this

[build]
command = "cargo leptos build --release && cargo install -q worker-build && LEPTOS_OUTPUT_NAME=my-new-project worker-build --release --features=ssr"
# Update LEPTOS_OUTPUT_NAME above
```

### 3. Update README.md

Replace the title and customize the project description with your own information.

### 4. Test the Build

```bash
cargo leptos build
```

## Customization Guide

After setup, here's how to customize the project:

### Adding New Pages

1. Create a new component in `src/pages/`:

```rust
// src/pages/about_page.rs
use leptos::prelude::*;
use leptos::{component, IntoView};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>About Us</h1>
            <p>Your content here</p>
        </div>
    }
}
```

2. Export it in `src/pages/mod.rs`:

```rust
pub mod about_page;
pub use about_page::AboutPage;
```

3. Add the route in `src/app.rs`:

```rust
use crate::pages::{HomePage, AboutPage};

#[component]
pub fn App() -> impl IntoView {
    // ... existing code ...
    view! {
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage />
                <Route path=StaticSegment("about") view=AboutPage />
            </Routes>
        </Router>
    }
}
```

### Styling

- Global styles: Edit `style/main.css`
- Component-specific styles: Add classes in your component `view!` macros
- Tailwind CSS: Optionally add to your project for utility-first styling

### Adding Server Functions (SSR Only)

Create async functions that run on the server:

```rust
#[server(GetUser)]
async fn get_user(id: u32) -> Result<String, ServerFnError> {
    // This runs on the server
    let user = fetch_from_db(id).await?;
    Ok(user.name)
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let user_data = create_resource(
        || (),
        |_| get_user(1)
    );

    view! {
        <Suspense fallback=|| "Loading...".into_view()>
            {move || user_data.get()}
        </Suspense>
    }
}
```

### Using Client-Side Only Mode

To remove SSR and use client-side rendering only:

1. Update `Cargo.toml` build features
2. Remove `bin-features = ["ssr"]`
3. Build with: `cargo leptos build --lib-features=hydrate`
4. Deploy to Cloudflare Pages instead of Workers

## Development Workflow

### Local Development

```bash
# Start development server with hot reload
cargo leptos watch

# Opens at http://127.0.0.1:8787
```

### Building

```bash
# Debug build
cargo leptos build

# Release build (optimized for size)
cargo leptos build --release
```

### Deployment

```bash
# Deploy to Cloudflare Workers
wrangler deploy
```

## Troubleshooting

### Build Fails

**Common issue:** Missing Rust targets

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install cargo-leptos if not present
cargo install cargo-leptos
```

**Common issue:** Wrangler not installed

```bash
# Install Wrangler CLI
npm install -g wrangler
# OR
cargo install wrangler
```

### Deployment Fails

1. Ensure you're logged into Cloudflare:
   ```bash
   wrangler login
   ```

2. Check `wrangler.toml` configuration is correct

3. Verify build succeeds before deploying:
   ```bash
   cargo leptos build --release
   ```

## Project Structure Reference

```
my-new-project/
├── src/
│   ├── app.rs              # App component with routing
│   ├── lib.rs              # SSR configuration
│   ├── main.rs             # Binary entrypoint
│   ├── worker_helpers.rs   # Cloudflare Workers helpers
│   └── pages/
│       ├── mod.rs          # Page exports
│       ├── home_page.rs    # Home page
│       └── about_page.rs   # Your new pages here
├── style/
│   └── main.css            # Global styles
├── scripts/
│   └── setup.sh            # Setup automation
├── Cargo.toml              # Rust configuration
├── wrangler.toml           # Cloudflare configuration
└── README.md               # Your project docs
```

## Resources

- **Leptos Documentation**: https://leptos.dev/
- **Leptos GitHub**: https://github.com/leptos-rs/leptos
- **Cloudflare Workers**: https://workers.cloudflare.com/
- **Axum Web Framework**: https://github.com/tokio-rs/axum
- **Wrangler CLI**: https://developers.cloudflare.com/workers/wrangler/

## Support

For issues or questions:

- Check the [Leptos Discord](https://discord.gg/leptos)
- Review [Cloudflare Workers docs](https://developers.cloudflare.com/workers/)
- Open an issue on [this template's GitHub](https://github.com/frontmesh/leptos-cloudflare-workers)

## License

This template is provided as-is for educational and development purposes.
