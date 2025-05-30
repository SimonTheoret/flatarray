vim.g.rustaceanvim = {
    server = {
        default_settings = {
            ["rust-analyzer"] = {
                cargo = {
                    allFeatures = true,
                },
            },
        },
    }
}
