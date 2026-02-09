# Netlify Oracle Server

This project acts as an Oracle Server for the Noir Circuit, providing external data (like ETH price) to the circuit via JSON-RPC. It is built with TypeScript and deployed as a serverless function on Netlify.

## Prerequisites

- **Node.js**: Install Node.js (v18+ recommended).
- **Netlify CLI**: Install globally via npm:
  ```bash
  npm install -g netlify-cli
  ```
- **Git**: Ensure you are in a git repository.

## Project Structure

- `src/api/index.ts`: The main Express application logic.
- `netlify/functions/api.ts`: Not used directly in source (except for development structure), but acts as the entry point wrapper for Netlify Functions when compiled.
- `netlify.toml`: Configuration for Netlify deployment (build commands, redirects).
- `tsconfig.json`: TypeScript compiler configuration.

## Setup & Installation

1.  **Install Dependencies**:
    Navigate to the server directory and install the required packages.
    ```bash
    cd 2-oracle-server
    npm install
    ```

## Development

To run the server locally for development:

1.  **Start the Server**:
    ```bash
    npm run dev
    ```
    This uses `nodemon` to watch for changes and restart the server.
    The server will be available at `http://localhost:5555`.

2.  **Testing**:
    You can send a JSON-RPC request to test functionality:
    ```bash
    curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "resolve_foreign_call", "params": [{"function": "fetchEthPrice", "inputs": []}], "id": 1}' http://localhost:5555
    ```

## Building & Deploying to Netlify

This project uses TypeScript, so it must be compiled before deployment. Netlify runs the build command specified in `netlify.toml`.

### Step-by-Step Deployment Guide

1.  **Login to Netlify**:
    ```bash
    netlify login
    ```

2.  **Initialize/Link Site**:
    If this is your first time deploying this folder:
    ```bash
    npx netlify deploy --prod
    ```
    - Select **"Create & configure a new project"**.
    - Choose your team.
    - Give it a name (or leave blank).
    - It will link the site and deploy.

3.  **Manual Build (Optional)**:
    You can verify the build locally before deploying:
    ```bash
    npm run build
    ```
    This compiles the TypeScript files into the `dist` directory.
    - **Note**: Ensure `tsconfig.json` has `"declaration": false` to prevent `.d.ts` files from being generated in the functions folder, which can cause deployment errors ("Incorrect function names").

4.  **Redeploying**:
    After making changes, simply run:
    ```bash
    npx netlify deploy --prod
    ```
    This will:
    - Run `npm run build` (as defined in `netlify.toml`).
    - Upload the functions from `dist/netlify/functions`.
    - Publish the site.

## Troubleshooting

- **"Incorrect function names" Error**:
    - This happens if non-JS/TS files (like `.d.ts` or `.map`) end up in the functions directory.
    - **Fix**: Ensure `tsconfig.json` has `"declaration": false` and `"sourceMap": false` (or strictly manage output files).
    - **Fix**: Run `git clean -fdX` to remove old build artifacts before rebuilding.

- **"Connect" Error in Noir Tests**:
    - Ensure the URL passed to `--oracle-resolver` matches your deployed Netlify URL (e.g., `https://your-site.netlify.app/`).

## API Endpoints

- **`POST /`** (or `/.netlify/functions/api`): The main JSON-RPC endpoint.
- **`GET /test`**: A simple health check endpoint.
