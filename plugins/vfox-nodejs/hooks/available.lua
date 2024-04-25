local json = require("json")
local http = require("http")
local util = require("util")

--- Return all available versions provided by this plugin
--- @param ctx table Empty table used as context, for future extension
--- @return table Descriptions of available versions and accompanying tool descriptions
available_result = nil
function PLUGIN:Available(ctx)
    if available_result then
        return available_result
    end
    local resp, err = http.get({
        url = util.getBaseUrl() .. util.VersionSourceUrl
    })
    if err ~= nil or resp.status_code ~= 200 then
        return {}
    end
    local body = json.decode(resp.body)
    local result = {}

    for _, v in ipairs(body) do
        table.insert(result, {
            version = string.gsub(v.version, "^v", ""),
            note = v.lts and "LTS" or "",
            addition = {
                {
                    name = "npm",
                    version = v.npm,
                }
            }
        })
    end
    table.sort(result, util.compare_versions)
    available_result = result
    return result
end