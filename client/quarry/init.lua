local packets = require("quarry.packets")

return function (quarry_config)
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
end