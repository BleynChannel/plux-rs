function circle(message)
	print(message..":")
	print("#*#")
	print("***")
	print("#*#")
end

return {
	{ name = "circle", inputs = {"message"}, func = circle }
}