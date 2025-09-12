function handle_request(user_id)
    -- Get user data
    local user_data = get_user_data(user_id)
    
    user_data = {
        user_id,
        user_data[1], -- name
        user_data[2] -- age
    }
    
    -- Render banner
    local banner = render_banner(user_data)

    -- Render content
    local content = [[
    <body>
        {{banner}}
        
        <h2>Administration Panel</h2>
        
        <div style="margin-top: 20px;">
            <div style="display: flex; gap: 20px; margin-bottom: 20px;">
                <div style="flex: 1; border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
                    <h3>User Management</h3>
                    <p>Manage user accounts, permissions, and access levels.</p>
                    <button style="padding: 8px 15px; background-color: #3498db; color: white; border: none; border-radius: 3px;">Manage Users</button>
                </div>
                
                <div style="flex: 1; border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
                    <h3>System Settings</h3>
                    <p>Configure application settings and preferences.</p>
                    <button style="padding: 8px 15px; background-color: #3498db; color: white; border: none; border-radius: 3px;">System Settings</button>
                </div>
            </div>
            
            <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
                <h3>Recent Activity</h3>
                <ul>
                    <li>User John Doe logged in at 10:30 AM</li>
                    <li>User Jane Smith updated profile at 9:15 AM</li>
                    <li>New user registration: Bob Johnson at 8:45 AM</li>
                </ul>
            </div>
        </div>
    </body>
    ]]

    return content
end

return {}