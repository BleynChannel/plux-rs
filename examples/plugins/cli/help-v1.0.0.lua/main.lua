function handle_command(command, args)
    if command == "help" then
        print("Available commands:")
        print("  help          - Show this help message")
        print("  user-info     - Display user information")
        print("  exit          - Exit the application")
        print("")
        print("For more information on a specific command, try:")
        print("  help <command>")
        return 0  -- Success exit code
    end
    
    return nil  -- Command not handled by this plugin
end

return {}