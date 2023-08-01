return {
    ClientConnect = {
        make_packet = function()
            local x, y, z = gps.locate()

            return {
                type = "ClientConnect",
                data = {
                    name = os.getComputerLabel() or "Name not set",
                    id = os.getComputerID(),
                    gps = { x, y, z },
                },
            }
        end,
    },
}
