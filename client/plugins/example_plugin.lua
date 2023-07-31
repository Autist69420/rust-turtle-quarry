return {
    name = "Example Plugin",
    description = "An example plugin",

    run = function(ws)
        print("Hello, Turtle, World!")
    end,

    on_register = function()
        print("Hey me me got registered!")
    end,
}
