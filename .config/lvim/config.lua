-- Read the docs: https://www.lunarvim.org/docs/configuration
-- Video Tutorials: https://www.youtube.com/watch?v=sFA9kX-Ud_c&list=PLhoH5vyxr6QqGu0i7tt_XoVK9v-KvZ3m6
-- Forum: https://www.reddit.com/r/lunarvim/
-- Discord: https://discord.com/invite/Xb9B4Ny

lvim.plugins = {
  { "catppuccin/nvim" },
  { "ionide/Ionide-vim" },
  { "nvim-lua/plenary.nvim" },
  -- { 'scalameta/nvim-metals', requires = { "nvim-lua/plenary.nvim" } },
  {
    "scalameta/nvim-metals",
    config = function()
      require("user.metals").config()
    end,
  },
  { "rcarriga/nvim-dap-ui", dependencies = { "mfussenegger/nvim-dap" } },
  { "folke/neodev.nvim",    opts = {} },
}


lvim.colorscheme = "catppuccin-latte"


vim.cmd([[nnoremap <S-h> :tabprevious<CR>]])

lvim.keys.normal_mode["L"] = ":BufferLineCycleNext<CR>"
lvim.keys.normal_mode["H"] = ":BufferLineCyclePrev<CR>"

print('hello from lua')

-- do: this settings allow for Ctrl + Backspace to delete words
-- vim.api.nvim_set_keymap('i', '<C-Backspace>', '<C-w>', {noremap = true, silent=true})
vim.api.nvim_set_keymap('i', '<C-H>', '<C-W>', { noremap = true })
-- i comment two lines below since they break switching to neovim tree by Ctrl + h
-- vim.api.nvim_set_keymap('n', '<C-H>', '<C-W>', {noremap = true})
-- vim.api.nvim_set_keymap('n', '<C-H>', 'a<C-w><ESC>', {noremap = true})
-- end

lvim.format_on_save = true

-- " Indents word-wrapped lines as much as the 'parent' line
vim.cmd("set breakindent")
-- " Ensures word-wrap does not split words
vim.cmd("set formatoptions=l")
vim.cmd("set lbr")


require("neodev").setup({
  library = { plugins = { "nvim-dap-ui" }, types = true },
})

local dap, dapui = require("dap"), require("dapui")
dap.listeners.after.event_initialized["dapui_config"] = function()
  dapui.open()
end
dap.listeners.before.event_terminated["dapui_config"] = function()
  dapui.close()
end
dap.listeners.before.event_exited["dapui_config"] = function()
  dapui.close()
end

dap.configurations.scala = {
  {
    type = "scala",
    request = "launch",
    name = "Run or Test Target",
    metals = {
      runType = "runOrTestFile",
    },
  },
  {
    type = "scala",
    request = "launch",
    name = "Test Target",
    metals = {
      runType = "testTarget",
    },
  },
}

-- vim.cmd("autocmd BufNewFile,BufRead * setlocal formatoptions-=cro")
vim.cmd("autocmd BufEnter * set formatoptions-=cro")
vim.cmd("autocmd BufEnter * setlocal formatoptions-=cro")
