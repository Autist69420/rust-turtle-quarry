local packets = require("quarry.packets")

local function handle_websocket_messages(ws)
    while true do
        local events = { os.pullEvent() }
        if events[1] == "websocket_message" then
        elseif events[1] == "websocket_close" then
        end
    end
end

return function(quarry_config)
    local ok, msg = http.checkURL(quarry_config.config.url)
    if not ok then
        return printError(("Could not contact URL: %s"):format(msg))
    end

    local ws, err = http.websocket(quarry_config.config.websocket_url)
    if ws and not err then
        print("[+] Connected to websocket!")
    else
        return printError(("Could not connect to websocket: %s"):format(err))
    end

    local client_connect_packet = packets.ClientConnect.make_packet()
    ws.send(textutils.serialiseJSON(client_connect_packet))

    local current_layer = 0

    if quarry_config.config.chunk_mode then
        print("[+] Chunk mode is enabled; Mining entire chunk, make sure to set it on a corner of a chunk")
    else
        local depth = quarry_config.config.mining.depth
        local width = quarry_config.config.mining.width
        local height = quarry_config.config.mining.height
        print(("[+] Chunk mode is disabled; Mining %dx%dx%d"):format(width, height, depth))
    end
end
