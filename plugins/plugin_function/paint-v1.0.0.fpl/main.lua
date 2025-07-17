function paint(is_circle)
	if is_circle then
		call_function_depend("circle", "1.0.0", "circle", "Hello world")
	else
		local is_exists, _ = call_function_optional_depend("square", "1.0.0", "square")
		if is_exists then
			print("Square function is successfully called")
		end
	end
end

return {
	{ name = "paint", inputs = {"is_circle"}, func = paint }
}