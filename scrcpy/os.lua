-- package.path = package.path .. ';./scrcpy/?.lua' --搜索lua模块
package.cpath = package.cpath .. ';./?.dll;'     --搜索dll模块

local json = require("dkjson")

local os_type = nil

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

local function get_w_load_percentage()
    local command = 'wmic cpu get loadpercentage'
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end

    local load_percentage = tonumber(string.match(output, "%d+"))
    return load_percentage
end

local function get_w_current_clock_speed()
    local command = 'wmic cpu get currentclockspeed'
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end

    local current_clock_speed = tonumber(string.match(output, "%d+"))
    return current_clock_speed
end

local function get_w_mem_info()
    local command = 'wmic OS get FreePhysicalMemory,TotalVisibleMemorySize /Value'
    local handle = io.popen(command)
    local output = handle and handle:read('*a') or ""

    if handle then
        handle:close()
    end

    local free_physical_memory = tonumber(string.match(output, 'FreePhysicalMemory=(%d+)'))
    local total_visible_memory_size = tonumber(string.match(output, 'TotalVisibleMemorySize=(%d+)'))

    return {
        free_physical_memory = free_physical_memory,
        total_visible_memory_size = total_visible_memory_size
    }
end



if package.config:sub(1, 1) == '\\' then -- windows
    -- 获取CPU利用率
    local load_percentage = get_w_load_percentage()

    -- 获取CPU速度
    local current_clock_speed = get_w_current_clock_speed()

    -- 获取系统正常运行时间
    local uptime = get_w_system_uptime()

    -- 获取内存信息
    local mem_info = get_w_mem_info()

    local ss = {
        cpu_load_percentage = load_percentage,
        cpu_clock_speed = current_clock_speed,
        system_uptime = uptime,
        free_physical_memory = mem_info.free_physical_memory,
        total_visible_memory_size = mem_info.total_visible_memory_size
    }
    local jsonString = json.encode(ss)
    print(jsonString)
    return ss;
elseif package.config:sub(1, 1) == '/' then -- unix linux
    -- 调用 top 命令，获取输出
    local f = io.popen("top -b -n 1")
    local output = f and f:read("*all") or ""
    if f then
        f:close()
    end

    -- 从输出中提取 CPU 和内存的信息
    local cpu_pattern =
    "Cpu%(s%):%s+(%d+%.%d+) us,%s+(%d+%.%d+) sy,%s+(%d+%.%d+) ni,%s+(%d+%.%d+) id,%s+(%d+%.%d+) wa,%s+(%d+%.%d+) hi,%s+(%d+%.%d+) si,%s+(%d+%.%d+) st"
    local mem_pattern = "KiB Mem :%s+(%d+) total,%s+(%d+) used,%s+(%d+) free,%s+(%d+) buffers"
    local cpu_info = { output:match(cpu_pattern) }
    local mem_info = { output:match(mem_pattern) }

    -- 打印 CPU 和内存的信息
    print("CPU usage:")
    print(string.format("User: %s%%", cpu_info[1]))
    print(string.format("System: %s%%", cpu_info[2]))
    print(string.format("Idle: %s%%", cpu_info[4]))
    print("Memory usage:")
    print(string.format("Total: %s KiB", mem_info[1]))
    print(string.format("Used: %s KiB", mem_info[2]))
    print(string.format("Free: %s KiB", mem_info[3]))
else
    os_type = "macOS" -- macos
end






return os_type
