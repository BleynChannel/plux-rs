local function BottomUpTree(depth)
    if depth > 0 then
        depth = depth - 1
        local left, right = BottomUpTree(depth), BottomUpTree(depth)
        return {left, right}
    else
        return {}
    end
end

local function ItemCheck(tree)
    if tree[1] then
        return 1 + ItemCheck(tree[1]) + ItemCheck(tree[2])
    else
        return 1
    end
end

function main(N)
	local mindepth = 4
	local maxdepth = mindepth + 2
	if maxdepth < N then
		maxdepth = N
	end

	do
		local stretchdepth = maxdepth + 1
		local stretchtree = BottomUpTree(stretchdepth)
	end

	local longlivedtree = BottomUpTree(maxdepth)

	for depth = mindepth, maxdepth, 2 do
		local iterations = 2 ^ (maxdepth - depth + mindepth)
		local check = 0
		for i = 1, iterations do
			check = check + ItemCheck(BottomUpTree(depth))
		end
	end
end

return {}