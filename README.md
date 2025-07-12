# VFox Plugin with Custom Backend Support

This VFox plugin library now supports custom backend operations, allowing vfox plugins to implement their own backend functionality that can be used by mise.

## Custom Backend Features

### Backend Metadata

Plugins can now declare backend support in their metadata:

```lua
PLUGIN = {
    name = "my-tool",
    version = "1.0.0",
    -- ... other metadata

    -- Enable custom backend support
    backendEnabled = true,
    backendName = "my-tool",  -- Optional custom backend name

    -- Backend operation implementations
    backend_list_versions = function(ctx)
        -- Return available versions
        return {
            versions = {"1.0.0", "1.1.0", "2.0.0"}
        }
    end,

    backend_install = function(ctx)
        -- Install the tool
        -- ctx.version - version to install
        -- ctx.install_path - where to install
        -- ctx.options - additional options

        return {
            success = true,
            message = "Installation successful"
        }
    end,

    backend_exec_env = function(ctx)
        -- Set environment variables
        return {
            env_vars = {
                {key = "MY_TOOL_ROOT", value = ctx.install_path},
                {key = "PATH", value = ctx.install_path .. "/bin"}
            }
        }
    end,

    backend_uninstall = function(ctx)
        -- Uninstall the tool
        return {
            success = true,
            message = "Uninstallation successful"
        }
    end
}
```

### Backend Operations

#### `backend_list_versions(ctx)`

Lists all available versions of the tool.

**Context:**

- `ctx.args` - Command line arguments
- `ctx.options` - Additional options

**Returns:**

- `versions` - Array of version strings

#### `backend_install(ctx)`

Installs a specific version of the tool.

**Context:**

- `ctx.args` - Command line arguments
- `ctx.version` - Version to install
- `ctx.install_path` - Installation directory
- `ctx.options` - Additional options

**Returns:**

- `success` - Boolean indicating success
- `message` - Optional status message

#### `backend_exec_env(ctx)`

Sets up environment variables for the tool.

**Context:**

- `ctx.args` - Command line arguments
- `ctx.version` - Installed version
- `ctx.install_path` - Installation directory
- `ctx.options` - Additional options

**Returns:**

- `env_vars` - Array of environment variable objects with `key` and `value`

#### `backend_uninstall(ctx)`

Uninstalls a specific version of the tool.

**Context:**

- `ctx.args` - Command line arguments
- `ctx.version` - Version to uninstall
- `ctx.install_path` - Installation directory
- `ctx.options` - Additional options

**Returns:**

- `success` - Boolean indicating success
- `message` - Optional status message

## Usage with mise

When a vfox plugin supports custom backend operations, mise will automatically use them instead of the default vfox behavior. This allows for:

1. **Custom version resolution** - Plugins can implement their own version listing logic
2. **Custom installation procedures** - Plugins can implement specialized installation steps
3. **Custom environment setup** - Plugins can set up tool-specific environment variables
4. **Custom cleanup** - Plugins can implement their own uninstallation logic

## Example Plugin

See `plugins/example-backend/` for a complete example of a vfox plugin with custom backend support.

## Development

To develop a plugin with custom backend support:

1. Set `backendEnabled = true` in your plugin metadata
2. Implement the desired backend operations as functions
3. Test your plugin with mise using the vfox backend

The plugin system will automatically detect and use your custom backend operations when available, falling back to standard vfox behavior when not implemented.
