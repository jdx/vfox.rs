--- Parse the legacy file found by vfox to determine the version of the tool.
--- Useful to extract version numbers from files like JavaScript's package.json or Golangs go.mod.
function PLUGIN:ParseLegacyFile(ctx)
    local filepath = ctx.filepath
    local file = io.open(filepath, "r")
    local content = file:read("*a")
    file:close()
    content = content:gsub("%s+", "")
    if content == "" then
        return {}
    end
    function resolve_legacy_version(strategy, query)
        local list = {}

        if strategy == "latest_installed" then
            list = ctx:getInstalledVersions()
        elseif strategy == "latest_available" then
            for _, av in pairs(self:Available({})) do
                table.insert(list, av.version)
            end
        else
            -- Just return the original query
            return query
        end

        local resolved = ""
        for _, item in pairs(list) do
            if item:match("^" .. query) then
                resolved = item
                break
            end
        end

        if resolved ~= "" then
            return resolved
        elseif strategy ~= "latest_available" then
            -- If no version is installed, fallback to latest_available
            return resolve_legacy_version("latest_available", query)
        else
            -- Give up and pretty the query itself
            return query
        end
    end
    local query = resolve_version(content)

    query = resolve_legacy_version("latest_installed", query)

    return {
        version = query
    }
end

function resolve_version(query)
    query = string.lower(query:gsub("v", ""))

    if query:match("^lts-") then
        query = query:gsub("-", "/")
    end

    local nodejs_codenames = {
        argon = 4,
        boron = 6,
        carbon = 8,
        dubnium = 10,
        erbium = 12,
        fermium = 14,
        gallium = 16,
        hydrogen = 18,
        iron = 20
    }

    for codename, version_number in pairs(nodejs_codenames) do
        if query == "lts/" .. codename then
            query = tostring(version_number)
            break
        end
    end

    if query == "lts" or query == "lts/*" then
        query = tostring(nodejs_codenames[#nodejs_codenames])
    end

    return query
end

