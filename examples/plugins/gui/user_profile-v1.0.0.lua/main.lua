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
    
    -- Create HTML-like content for the user profile
    local content = [[
<div>
    <h1>User Profile</h1>
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px; background-color: #f8f9fa;">
        <h2>]] .. user_data[1] .. [[</h2>
        <p>User ID: ]] .. user_id .. [[</p>
        <p>Age: ]] .. user_data[2] .. [[</p>
        <p>Status: <span style="color: green;">Online</span></p>
        <p>Last Login: <span>Today, 09:30 AM</span></p>
        <p>Member Since: <span>January 15, 2023</span></p>
    </div>
    
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px; background-color: #f8f9fa;">
        <h3>Contact Information</h3>
        <p>Email: <span>]] .. string.lower(string.gsub(user_data[1], " ", ".")) .. [[@example.com</span></p>
        <p>Phone: <span>+1 (555) 123-4567</span></p>
    </div>
    
    <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; background-color: #f8f9fa;">
        <h3>Profile Actions</h3>
        <button style="padding: 8px 15px; background-color: #3498db; color: white; border: none; border-radius: 3px; margin-right: 10px; cursor: pointer;">Edit Profile</button>
        <button style="padding: 8px 15px; background-color: #2ecc71; color: white; border: none; border-radius: 3px; margin-right: 10px; cursor: pointer;">Change Password</button>
        <button style="padding: 8px 15px; background-color: #f39c12; color: white; border: none; border-radius: 3px; cursor: pointer;">Privacy Settings</button>
    </div>
</div>
    ]]
    
    return content
end

-- Plugin metadata
return {
    name = "User Profile Plugin",
    version = "1.0.0",
    description = "A GUI plugin for displaying detailed user profile information"
}