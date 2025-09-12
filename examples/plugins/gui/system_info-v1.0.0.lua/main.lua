function render_ui(user_id)
    -- Validate input
    if not user_id or type(user_id) ~= "number" then
        return "<div>Error: Invalid user ID provided</div>"
    end
    
    -- Get system info with error handling
    local sys_info = get_system_info()
    if not sys_info or type(sys_info) ~= "table" then
        return "<div>Error: Unable to retrieve system information</div>"
    end
    
    -- Provide default values if keys are missing
    local os_name = sys_info["os"] or "Unknown"
    local arch = sys_info["arch"] or "Unknown"
    local hostname = sys_info["hostname"] or "localhost"
    
    -- Create HTML-like content for system information
    local content = [[
<div>
    <h1>System Information</h1>
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px; background-color: #f8f9fa;">
        <h2>Operating System</h2>
        <p>OS: ]] .. os_name .. [[</p>
        <p>Architecture: ]] .. arch .. [[</p>
        <p>Hostname: ]] .. hostname .. [[</p>
    </div>
    
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px; background-color: #f8f9fa;">
        <h2>Hardware</h2>
        <p>CPU: Intel Core i7-8550U @ 1.80GHz (8 cores)</p>
        <p>Memory: 16 GB DDR4 (2400 MHz)</p>
        <p>Storage: 512 GB SSD (NVMe)</p>
        <p>Graphics: NVIDIA GeForce GTX 1650</p>
    </div>
    
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px; background-color: #f8f9fa;">
        <h2>Network</h2>
        <p>IP Address: 192.168.1.100</p>
        <p>Subnet Mask: 255.255.255.0</p>
        <p>Gateway: 192.168.1.1</p>
        <p>DNS: 8.8.8.8, 8.8.4.4</p>
    </div>
    
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; background-color: #f8f9fa;">
        <h2>Software</h2>
        <p>Runtime: Lua 5.4</p>
        <p>Plugin System: Plux v1.0.0</p>
        <p>GUI Framework: egui</p>
    </div>
</div>
    ]]
    
    return content
end

-- Plugin metadata
return {
    name = "System Info Plugin",
    version = "1.0.0",
    description = "A GUI plugin for displaying comprehensive system information"
}