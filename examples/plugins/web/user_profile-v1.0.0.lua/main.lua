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
        
        <h2>User Profile</h2>
        <div style="margin-top: 20px; border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
            <h3>Profile Information</h3>
            <p>Name: {{user_data[1]}}</p>
            <p>Age: {{user_data[2]}}</p>
            <p>User ID: {{user_data[0]}}</p>
            <p>Status: <span style="color: green;">Online</span></p>
        </div>
        
        <div style="margin-top: 20px;">
            <button style="padding: 8px 15px; background-color: #3498db; color: white; border: none; border-radius: 3px; margin-right: 10px;">Edit Profile</button>
            <button style="padding: 8px 15px; background-color: #2ecc71; color: white; border: none; border-radius: 3px;">Change Password</button>
        </div>
    </body>
    ]]

    return content
end

return {}