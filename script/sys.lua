---@diagnostic disable: unused-function, undefined-global
local function test()
    local co = coroutine.create(
        function ()
            tyme_sys:send_markdown("test/test", "test1856")
        end
    )
    coroutine.resume(co)
    print("sending....");
end

return {
    test = test
}
