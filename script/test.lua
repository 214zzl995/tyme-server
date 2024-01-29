local sys = require("sys")
sys.send_markdown("test/test", "##测试")
print(sys.sys_config.log_location)
