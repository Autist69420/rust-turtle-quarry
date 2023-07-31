local current_dir = fs.getDir(shell.getRunningProgram())
local plugins_path = fs.combine(current_dir, "plugins")

local loaded_plugins = {}

local plugin_env = require("env.plugin").make_env()

local function register_plugin(plugin_path)
    -- TODO: A custom plugin ENV
    local plugin = require(plugin_path)
    print(("[+] Registering plugin: %s"):format(plugin.name))
    plugin_env[plugin.name] = plugin.run

    local status, err = pcall(plugin.on_register)
    if not status and err then
        printError(("[+] Failed to load plugin: %s"):format(plugin.name))
        printError(("    %s"):format(err))
        return
    end
end

if fs.exists(plugins_path) then
    print("[+] Registering plugins")
    local plugins = fs.list(plugins_path)
    for i, plugin in pairs(plugins) do
        local absolute_plugin_path = fs.combine(plugins_path, plugin)
        local lua_extension_removed = absolute_plugin_path:gsub(".lua", "")
        register_plugin(lua_extension_removed)
    end
end