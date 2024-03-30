-- package.path = package.path .. ';./scrcpy/?.lua' --搜索lua模块
package.cpath = package.cpath .. ';./?.dll;' --搜索dll模块

local sys = require("tyme_sys")

local os_type = nil

-- 获取 Windows 系统运行时间
local function get_w_system_uptime()
    local command = 'wmic os get lastbootuptime /format:list'
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end

    -- 解析输出，提取最后启动时间
    local lastBootTime = string.match(output, 'LastBootUpTime=(.+)')

    if lastBootTime then
        -- 将最后启动时间转换为日期对象
        local year = tonumber(string.sub(lastBootTime, 1, 4))
        local month = tonumber(string.sub(lastBootTime, 5, 6))
        local day = tonumber(string.sub(lastBootTime, 7, 8))
        local hour = tonumber(string.sub(lastBootTime, 9, 10))
        local minute = tonumber(string.sub(lastBootTime, 11, 12))
        local second = tonumber(string.sub(lastBootTime, 13, 14))

        -- 计算正常运行时间
        local uptime = os.time() -
            os.time({ year = year, month = month, day = day, hour = hour, min = minute, sec = second })
        return uptime
    end

    return nil
end

-- 获取 Windows CPU 负载百分比
local function get_w_cpu_load_percentage()
    local command = 'wmic cpu get loadpercentage'
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end

    local load_percentage = tonumber(string.match(output, "%d+"))
    return load_percentage
end

-- 获取 Windows CPU 时钟速度
local function get_w_cpu_clock_speed()
    local command = 'wmic cpu get currentclockspeed'
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end

    local current_clock_speed = tonumber(string.match(output, "%d+"))
    return current_clock_speed
end

-- 获取 Windows 系统总内存大小
local function get_w_total_memory_size()
    local command = 'wmic OS get TotalVisibleMemorySize /Value'
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end

    local total_memory_size = tonumber(string.match(output, 'TotalVisibleMemorySize=(%d+)'))

    return total_memory_size
end

-- 获取Windows 空闲物理内存
local function get_w_free_memory_size()
    local command = "wmic OS get FreePhysicalMemory /Value"
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end
    local total_memory_size
    if output ~= "" then
        total_memory_size = tonumber(string.match(output, 'FreePhysicalMemory=(%d+)'))
    else
        total_memory_size = 0
    end

    return total_memory_size
end



-- 获取 Linux CPU 负载百分比
local function get_l_cpu_load_percentage()
    local command = "top -bn1 | grep 'Cpu(s)' | awk '{print $2 + $4}'"
    local handle = io.popen(command)
    local result = handle and handle:read("*a") or ""
    if handle then
        handle:close()
    end
    return tonumber(result)
end

-- 获取 Linux CPU 时钟速度 (虚拟机好像没法正常获取到)
local function get_l_cpu_clock_speed()
    local command = "lscpu | grep 'CPU MHz' | awk '{print $3}'"
    local handle = io.popen(command)
    local result = handle and handle:read("*a") or ""
    if handle then
        handle:close()
    end
    return tonumber(result)
end

-- 获取 Linux 系统运行时间
local function get_l_system_uptime()
    local command = "awk '{print $1}' /proc/uptime"
    local handle = io.popen(command)
    local result = handle and handle:read("*a") or ""
    if handle then
        handle:close()
    end
    return tonumber(result)
end

-- 获取Linux 空闲物理内存
local function get_l_free_memory_size()
    local command = "grep -E '^MemFree:' /proc/meminfo | awk '{print $2}'"
    local handle = io.popen(command)
    local result = handle and handle:read("*a") or ""
    if handle then
        handle:close()
    end
    return tonumber(result)
end

-- 获取总可见内存大小
local function get_l_total_memory_size()
    local command = "grep -E '^MemTotal:' /proc/meminfo | awk '{print $2}'"
    local handle = io.popen(command)
    local result = handle and handle:read("*a") or ""
    if handle then
        handle:close()
    end
    return tonumber(result)
end

local os_info = nil

if package.config:sub(1, 1) == '\\' then -- windows
    os_type = "windows"                  -- windows
    os_info = {
        cpu_load_percentage = get_w_cpu_load_percentage(),
        cpu_clock_speed = get_w_cpu_clock_speed(),
        system_uptime = get_w_system_uptime(),
        free_memory_size = get_w_free_memory_size(),
        total_memory_size = get_w_total_memory_size()
    }
elseif package.config:sub(1, 1) == '/' then -- unix linux
    os_type = "linux"                       -- linux
    os_info = {
        cpu_load_percentage = get_l_cpu_load_percentage(),
        cpu_clock_speed = get_l_cpu_clock_speed(),
        system_uptime = get_l_system_uptime(),
        free_memory_size = get_l_free_memory_size(),
        total_memory_size = get_l_total_memory_size()
    }
else
    os_type = "macOS" -- macos
end

sys.send_json("system/info", os_info)
