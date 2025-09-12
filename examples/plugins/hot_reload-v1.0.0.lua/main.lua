local step = 4

function fibonacci(n)
    if n <= 1 then
        return n
    end
    
    local a, b = 0, 1
    for i = 2, n do
        a, b = b, a + b
    end
    return b
end

function status()
    -- Закоментируйте эту строку...
    -- return get_timestamp()

    -- И раскоментируйте эти строки, чтобы увидеть изменение
    step = step + 1
    return fibonacci(step)
end

return {}