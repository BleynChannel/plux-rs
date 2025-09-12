function handle_command(command, args)
    if command == "user-info" then
        if #args == 0 then
            print("Usage: user-info <username>")
            return 1  -- Error exit code
        end
        
        local username = args[1]
        local user_data = get_user_info(username)
        
        if user_data[1] == "Unknown" then
            print("User '" .. username .. "' not found")
            return 1  -- Error exit code
        else
            print("User Information:")
            print("  Username: " .. username)
            print("  Name: " .. user_data[1])
            print("  Age: " .. user_data[2])
            return 0  -- Success exit code
        end
    end
    
    return nil  -- Command not handled by this plugin
end

return {}