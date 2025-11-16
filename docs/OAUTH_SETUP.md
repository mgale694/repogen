# OAuth Setup Guide

This guide explains how to set up GitHub OAuth authentication for repogen.

## Why OAuth?

OAuth provides several advantages over Personal Access Tokens (PAT):

- ✅ More secure - tokens are managed by GitHub
- ✅ Scoped permissions - only request what you need
- ✅ Automatic browser authentication
- ✅ Better user experience

## Current Status

✅ **OAuth Device Flow is fully active and ready to use!**

OAuth authentication is now the recommended method for repogen. The CLI provides a complete interactive guide for creating your GitHub OAuth App - no code editing or manual configuration required!

## Setting Up OAuth

OAuth authentication is now **fully interactive**! No code editing required.

### Quick Start

Simply run:

```bash
repogen init --auth
```

Then select "OAuth Login (Browser)" when prompted. repogen will guide you through the entire setup process!

### Interactive Setup Process

When you choose OAuth authentication for the first time, repogen will:

1. **Ask if you want to set up OAuth** (one-time setup, ~2 minutes)
2. **Open your browser** to GitHub's developer settings automatically
3. **Show you step-by-step instructions** for creating an OAuth App:
   - Application name: `repogen`
   - Homepage URL: Your GitHub repo URL
   - Callback URL: `http://127.0.0.1`
   - Enable Device Flow checkbox
4. **Prompt you for the Client ID** from your new OAuth App
5. **Save the Client ID** to your config file (`~/.config/repogen/config.toml`)
6. **Immediately start the OAuth flow** to authenticate

### Complete Example

```bash
$ repogen init --auth

🔐 Step 3: GitHub Authentication
? How would you like to authenticate with GitHub? OAuth Login (Browser)

🌐 OAuth Browser Authentication
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📋 OAuth Setup Required

OAuth authentication requires a GitHub OAuth App.
This is a one-time setup that takes about 2 minutes.

? Would you like to set up OAuth now? Yes

📝 OAuth App Setup Guide
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step 1: Create a GitHub OAuth App
   → Open: https://github.com/settings/developers

✅ Browser opened automatically

Step 2: Click 'New OAuth App' (or 'Register a new application')

Step 3: Fill in the application details:
   • Application name: repogen
   • Homepage URL: https://github.com/yourusername/repogen
   • Authorization callback URL: http://127.0.0.1
   • Description: CLI tool for GitHub repository creation

Step 4: After creating the app:
   • Check the box: ☑️  Enable Device Flow
   • Copy the Client ID (starts with 'Iv1.' or similar)

? Press Enter when you've created the app and have the Client ID ready

📋 Enter OAuth App Details

? GitHub OAuth App Client ID: Iv1.abc123def456...

✅ Client ID saved to config!

🎉 OAuth setup complete! Now let's authenticate...

📝 Requesting device code from GitHub...

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  Please visit: https://github.com/login/device   ┃
┃  And enter code: WDJB-MJHT                       ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

✅ Browser opened automatically
⠋ Waiting for authorization...
✅ Authorization successful!
✅ Successfully authenticated with GitHub!
```

### Manual Setup (Alternative)

If you prefer to set up the OAuth App manually first:

1. **Create OAuth App**:

   - Go to https://github.com/settings/developers
   - Click "New OAuth App"
   - Fill in the details as shown above
   - Enable Device Flow
   - Copy the Client ID

2. **Add to config manually** (optional):
   Edit `~/.config/repogen/config.toml` and add:

   ```toml
   oauth_client_id = "Iv1.your_client_id_here"
   ```

3. **Run authentication**:
   ```bash
   repogen init --auth
   ```
   Select OAuth and it will use your configured Client ID

## How OAuth Device Flow Works

When a user runs `repogen init --auth` and selects OAuth:

1. **Device Code Request**: repogen requests a device code from GitHub
2. **User Prompt**: The CLI displays a code and opens the browser to https://github.com/login/device
3. **User Authorization**: User enters the code in their browser and authorizes the app
4. **Token Polling**: repogen polls GitHub's token endpoint until authorization is complete
5. **Token Storage**: Once authorized, the access token is securely saved to `~/.config/repogen/config.toml`

### Example Flow

```
🌐 Using OAuth browser authentication
📝 Requesting device code from GitHub...

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  Please visit: https://github.com/login/device  ┃
┃  And enter code: WDJB-MJHT           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

✅ Browser opened automatically
⠋ Waiting for authorization...
✅ Authorization successful!
✅ Successfully authenticated with GitHub!
```

## Security Considerations

### Client Secret

Note that the Device Flow **does not require a client secret**. This makes it safe for CLI applications because:

- The client ID can be public (it's visible in the code)
- No secrets need to be embedded in the binary
- Users authenticate via GitHub's website, not through the CLI

### Token Storage

Access tokens are stored in `~/.config/repogen/config.toml` with file permissions set to be readable only by the user (0600 on Unix systems).

## Managing OAuth Configuration

### View Your OAuth Setup

```bash
repogen config --view
```

This shows your configured OAuth Client ID (partially masked for security).

### Reconfigure OAuth

If you need to change your OAuth App or Client ID:

```bash
repogen config --edit
```

Or clear all config and start over:

```bash
repogen config --clear
repogen init --auth
```

### Using Multiple OAuth Apps

You can switch between different OAuth Apps (e.g., personal vs. organization):

1. Edit your config:
   ```bash
   repogen config --edit
   ```
2. The Client ID is stored in `~/.config/repogen/config.toml` as:
   ```toml
   oauth_client_id = "Iv1.your_client_id"
   ```
3. Or manually edit the file to switch Client IDs

## Troubleshooting

### "OAuth client ID not configured"

This means you haven't completed the OAuth setup yet. Run:

```bash
repogen init --auth
```

and select OAuth when prompted.

### "Device code expired"

Device codes expire after 15 minutes. If the user doesn't authorize in time, they'll need to run `repogen init --auth` again.

### "Access denied"

This happens when the user clicks "Cancel" during authorization. They can try again by running the init command.

### "Slow down" errors

If you see these errors, the polling interval is being respected. The CLI automatically adjusts when GitHub requests slower polling.

### "Invalid Client ID"

Double-check that:

- You copied the entire Client ID (usually starts with `Iv1.`)
- Device Flow is enabled in your OAuth App settings
- The OAuth App is properly registered on GitHub

### Rate Limits

- **50 device code verifications per hour** per application
- Polling respects GitHub's requested interval (typically 5 seconds)
- Automatic backoff when "slow_down" response is received

## For Production Use

When deploying repogen for public use, consider:

1. **Register a public OAuth app** under an organization account
2. **Document the Client ID** in the README
3. **Request minimal scopes** (currently: `repo user`)
4. **Add token refresh logic** (GitHub tokens don't expire by default, but can be revoked)
5. **Implement proper error handling** for network issues
6. **Add telemetry** (with user consent) to track OAuth success rates

## Alternative: Using Personal Access Tokens

Until OAuth is configured, users can create a Personal Access Token:

1. Visit https://github.com/settings/tokens/new
2. Name: `repogen`
3. Scopes: `repo`, `user`
4. Generate token
5. Run `repogen init --auth` and paste the token

PATs work immediately without any app registration and are perfect for personal use or testing.

## Technical Implementation

### How It Works

The interactive OAuth setup is fully implemented in repogen with no code editing required:

#### 1. Configuration Storage

OAuth Client ID is stored in `~/.config/repogen/config.toml`:

```toml
oauth_client_id = "Iv1.your_client_id_here"
```

This is set automatically during the guided setup process.

#### 2. Smart Flow Control

When `repogen init --auth` is run:

1. **Check configuration**: Is `oauth_client_id` present?

   - **No**: Offer guided setup → Create OAuth App → Save Client ID → Authenticate
   - **Yes**: Skip setup → Use saved Client ID → Authenticate

2. **Fallback options**: At any point, users can switch to PAT authentication

3. **Error recovery**: Clear messages with retry options if OAuth fails

#### 3. OAuth Device Flow Process

Once Client ID is configured:

1. **Request device code**: POST to `https://github.com/login/device/code`
2. **Display user code**: Show code and verification URL
3. **Open browser**: Automatically open GitHub verification page
4. **Poll for token**: POST to `https://github.com/login/oauth/access_token`
5. **Save token**: Store access token in config file

#### 4. Security Considerations

- **No client secrets**: Device Flow doesn't require secrets (safe for CLI apps)
- **Client ID is public**: Can be safely committed to code or config
- **User authenticates on GitHub**: Not in the CLI itself
- **Token storage**: Standard config file with user-only permissions

### API Endpoints Used

| Endpoint                                           | Purpose                                     |
| -------------------------------------------------- | ------------------------------------------- |
| `POST https://github.com/login/device/code`        | Request device and user verification codes  |
| `POST https://github.com/login/oauth/access_token` | Poll for authorization and get access token |
| `GET https://api.github.com/user`                  | Validate token and get user info            |

### Code Structure

The implementation is organized into focused methods:

- `handle_oauth_authentication()` - Main OAuth flow coordinator
- `guide_oauth_setup()` - Interactive OAuth App creation guide
- `run_device_flow()` - GitHub Device Flow implementation
- `validate_github_token()` - Token validation against GitHub API

All code is in `src/commands/init.rs` with configuration management in `src/utils/config.rs`.

## Comparison: OAuth vs PAT

| Feature              | OAuth                         | Personal Access Token        |
| -------------------- | ----------------------------- | ---------------------------- |
| **Setup Time**       | ~2 minutes (one-time)         | 30 seconds                   |
| **Future Auth**      | Browser click                 | Token entry                  |
| **User Experience**  | Excellent                     | Good                         |
| **Code Editing**     | None required                 | None required                |
| **Best For**         | Teams, organizations, best UX | Personal use, quickest start |
| **Token Management** | GitHub manages                | User manages                 |
| **Revocation**       | Via GitHub UI                 | Via GitHub UI                |

## References

- [GitHub OAuth Apps Documentation](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps)
- [OAuth 2.0 Device Authorization Grant (RFC 8628)](https://tools.ietf.org/html/rfc8628)
- [GitHub API - Device Flow](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow)
