# Deployment Setup for itch.io

## Prerequisites

1. **itch.io Account**: Create an account at https://itch.io
2. **Game Project**: Create a new game project on itch.io
3. **API Key**: Generate API key from https://itch.io/user/settings/api-keys

## GitHub Secrets Setup

Add the following secrets to your GitHub repository:

1. Go to your GitHub repository
2. Navigate to Settings > Secrets and variables > Actions
3. Add the following repository secrets:

### Required Secrets:

- `BUTLER_API_KEY`: Your itch.io API key
- `ITCH_USER`: Your itch.io username
- `ITCH_GAME`: Your game project name on itch.io

### Example:
```
BUTLER_API_KEY: your-api-key-here
ITCH_USER: yourusername
ITCH_GAME: reversi
```

## How it Works

The CI/CD pipeline consists of three main jobs:

### 1. Test Job
- Runs on every push and pull request
- Installs Rust and dependencies
- Runs tests, format checks, and clippy
- Must pass before build job runs

### 2. Build Job
- Runs after test job passes
- Builds for Windows, Linux, and macOS
- Creates release binaries for each platform
- Uploads build artifacts

### 3. Deploy Job
- Runs only on main branch after successful build
- Downloads all build artifacts
- Packages builds for itch.io
- Uses Butler (itch.io CLI) to deploy to itch.io
- Creates separate channels for each platform

## Platform Channels

The deployment creates the following channels on itch.io:
- `windows`: Windows executable
- `linux`: Linux executable  
- `macos`: macOS executable

## Manual Deployment

If you need to deploy manually:

1. Install Butler CLI:
```bash
curl -L -o butler.zip https://broth.itch.ovh/butler/linux-amd64/LATEST/archive/default
unzip butler.zip
chmod +x butler
```

2. Login to itch.io:
```bash
./butler login
```

3. Build and deploy:
```bash
cargo build --release
./butler push target/release/reversi yourusername/yourgame:platform
```

## Troubleshooting

- **API Key Issues**: Make sure your API key is valid and has the correct permissions
- **Build Failures**: Check the Actions tab for detailed error logs
- **Platform Issues**: Ensure your game supports the target platforms
- **Butler Errors**: Check itch.io status and Butler documentation

## Notes

- Deployment only happens on pushes to the main branch
- Each platform gets its own channel on itch.io
- The version is set to the Git commit SHA
- Butler automatically handles file uploads and channel management