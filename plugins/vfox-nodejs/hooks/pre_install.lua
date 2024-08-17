local http = require("http")
local util = require("util")
--- Returns some pre-installed information, such as version number, download address, local files, etc.
--- If checksum is provided, vfox will automatically check it for you.
--- @param ctx table
--- @field ctx.version string User-input version
--- @return table Version information
function PLUGIN:PreInstall(ctx)
    local version = ctx.version

    if version == "latest" then
        local lists = self:Available({})
        version = lists[1].version
    end

    if not util.is_semver_simple(version) then
        local lists = self:Available({})
        local shorthands = util.calculate_shorthand(lists)
        version = shorthands[version]
    end

    if (version == nil) then
        error("version not found for provided version " .. (ctx.version or "null"))
    end

    local arch_type = RUNTIME.archType
    local ext = ".tar.gz"
    local osType = RUNTIME.osType
    if RUNTIME.archType == "amd64" then
        arch_type = "x64"
    end
    if RUNTIME.osType == "windows" then
        ext = ".zip"
        osType = "win"
    end
    -- add logic for macOS M1~
    if RUNTIME.osType == "darwin" then
        local major, _ = util.extract_semver(version)
        if major and tonumber(major) <= 16 then
            arch_type = "x64"
        end
    end

    local filename = util.FileName:format(version, osType, arch_type, ext)
    local baseUrl = util.getBaseUrl() .. util.NodeBaseUrl:format(version)

    local resp, err = http.get({
        url = baseUrl .. "SHASUMS256.txt"
    })
    if err ~= nil or resp.status_code ~= 200 then
        error("get checksum failed")
    end
    local checksum = util.get_checksum(resp.body, filename)
    return {
        version = version,
        url = baseUrl .. filename,
        sha256 = checksum,
    }
end
