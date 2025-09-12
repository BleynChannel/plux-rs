function render_ui(user_id)
    -- Validate input
    if not user_id or type(user_id) ~= "number" then
        return "<div>Error: Invalid user ID provided</div>"
    end
    
    -- Get user data with error handling
    local user_data = get_user_data(user_id)
    if not user_data or type(user_data) ~= "table" or #user_data < 2 then
        return "<div>Error: Unable to retrieve user data</div>"
    end
    
    -- Create HTML-like content for the dashboard
    local content = [[
<div>
    <h1>Dashboard</h1>
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px; background-color: #f8f9fa;">
        <h2>Welcome, ]] .. user_data[1] .. [[!</h2>
        <p>User ID: ]] .. user_id .. [[</p>
        <p>Age: ]] .. user_data[2] .. [[</p>
        <p>Status: <span style="color: green;">Online</span></p>
    </div>
    
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px; background-color: #f8f9fa;">
        <h3>System Statistics</h3>
        <p>Total Users: <strong>1,243</strong></p>
        <p>Active Sessions: <strong>57</strong></p>
        <p>Recent Logins: <strong>23</strong></p>
        <p>System Load: <strong>23%</strong></p>
    </div>
    
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; background-color: #f8f9fa;">
        <h3>Quick Actions</h3>
        <button style="padding: 8px 15px; background-color: #3498db; color: white; border: none; border-radius: 3px; margin-right: 10px; cursor: pointer;">View Reports</button>
        <button style="padding: 8px 15px; background-color: #2ecc71; color: white; border: none; border-radius: 3px; margin-right: 10px; cursor: pointer;">Manage Users</button>
        <button style="padding: 8px 15px; background-color: #e74c3c; color: white; border: none; border-radius: 3px; cursor: pointer;">System Settings</button>
    </div>
</div>
    ]]
    
    return content
end

-- Plugin metadata
return {
    name = "Dashboard Plugin",
    version = "1.0.0",
    description = "A GUI dashboard plugin showing user statistics and system information"
}