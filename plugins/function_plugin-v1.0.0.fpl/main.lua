function mul(a, b)
	return a * b;
end

function main()
	print("4 + 6 = " .. add(4, 6));
	print("9 - 3 = " .. sub(9, 3));
	print("8 * 3 = " .. mul(8, 3));
end

function echo(message)
	return "Message v.1.0.0: " .. message;
end

return {}