#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Leptos Cloudflare Workers - Project Setup${NC}"
echo -e "${BLUE}============================================${NC}\n"

# Check if project name argument was provided
if [ -z "$1" ]; then
    read -p "Enter your project name (kebab-case, e.g., my-awesome-app): " PROJECT_NAME
else
    PROJECT_NAME="$1"
fi

# Validate project name
if [ -z "$PROJECT_NAME" ]; then
    echo -e "${YELLOW}❌ Project name cannot be empty${NC}"
    exit 1
fi

# Check if name is already the template name
if [ "$PROJECT_NAME" = "leptos-cloudflare-workers" ]; then
    echo -e "${YELLOW}⚠️  Project name is the same as the template. Please choose a different name.${NC}"
    exit 1
fi

echo -e "\n${BLUE}Updating project configuration...${NC}\n"

# Update Cargo.toml
echo -e "${BLUE}→${NC} Updating Cargo.toml"
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s/leptos-cloudflare-workers/$PROJECT_NAME/g" Cargo.toml
else
    sed -i "s/leptos-cloudflare-workers/$PROJECT_NAME/g" Cargo.toml
fi

# Update wrangler.toml
echo -e "${BLUE}→${NC} Updating wrangler.toml"
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s/leptos-cloudflare-workers/$PROJECT_NAME/g" wrangler.toml
else
    sed -i "s/leptos-cloudflare-workers/$PROJECT_NAME/g" wrangler.toml
fi

# Update README.md (only project structure and title)
echo -e "${BLUE}→${NC} Updating README.md"
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "s/leptos-cloudflare-workers/$PROJECT_NAME/g" README.md
    sed -i '' "s/# Leptos Cloudflare Implementation/# $PROJECT_NAME/g" README.md
else
    sed -i "s/leptos-cloudflare-workers/$PROJECT_NAME/g" README.md
    sed -i "s/# Leptos Cloudflare Implementation/# $PROJECT_NAME/g" README.md
fi

echo -e "\n${GREEN}✅ Project setup complete!${NC}\n"
echo -e "${GREEN}Your project '${PROJECT_NAME}' is ready to go!${NC}\n"

echo -e "${BLUE}Next steps:${NC}"
echo -e "  1. ${YELLOW}cargo leptos build${NC}       - Build the project"
echo -e "  2. ${YELLOW}cargo leptos watch${NC}        - Start development server"
echo -e "  3. ${YELLOW}wrangler deploy${NC}           - Deploy to Cloudflare Workers"
echo -e "\n${BLUE}Documentation:${NC}"
echo -e "  • README.md for project overview"
echo -e "  • Leptos docs: https://leptos.dev/"
echo -e "  • Cloudflare Workers: https://workers.cloudflare.com/"
echo ""
