local source = '/home/john/testimage.png'
local buf = vim.api.nvim_get_current_buf()
local image = require('hologram.image'):new(source, {})

print(buf)

-- Image should appear below this line, then disappear after 5 seconds

image:display(5, 0, buf, {})

vim.defer_fn(function()
    image:delete(0, {free = true})
end, 5000)
