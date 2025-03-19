return {
    ["run_plugin"] = function()
        print("Pokemon party slot 1", readbyte(0xda23))
    end,
    ["permissions"] = {
        "readbyte"
    },
}
