---@diagnostic disable: unused-function, undefined-global
local function send_markdown(topic, content, callback)
    local co = coroutine.create(
        function()
            tyme_sys:send_markdown(topic, content)
            if callback then
                callback()
            end
        end
    )
    coroutine.resume(co)
end

local function send_json(topic, content, callback)
    local co = coroutine.create(
        function()
            tyme_sys:send_json(topic, content)
            if callback then
                callback()
            end
        end
    )
    coroutine.resume(co)
end

local sys_config = tyme_sys.sys_config

return {
    send_markdown = send_markdown,
    send_json = send_json,
    sys_config = sys_config
}
