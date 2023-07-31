return {
    ClientConnect = {
        make_packet = function()
            return {
                type = "ClientConnect",
                data = {
                    name = os.getComputerLabel() or "Name not set",
                    id = os.getComputerID(),
                },
            }
        end,
    },
}
