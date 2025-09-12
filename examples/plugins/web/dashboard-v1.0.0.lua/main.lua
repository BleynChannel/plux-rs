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
        
        <h2>Dashboard</h2>
        <div style="margin-top: 20px;">
            <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px; margin-bottom: 10px;">
                <h3>System Statistics</h3>
                <p>Total Users: <strong>1,243</strong></p>
                <p>Active Sessions: <strong>57</strong></p>
                <p>Recent Logins: <strong>23</strong></p>
            </div>
            
            <div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
                <h3>Quick Actions</h3>
                <button style="padding: 8px 15px; background-color: #3498db; color: white; border: none; border-radius: 3px; margin-right: 10px;">View Reports</button>
                <button style="padding: 8px 15px; background-color: #2ecc71; color: white; border: none; border-radius: 3px;">Manage Users</button>
            </div>
        </div>
    </body>
    ]] 

    return content
end

return {}