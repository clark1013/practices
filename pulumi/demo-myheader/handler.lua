local MyHeader = {}

MyHeader.PRIORITY = 1000

function MyHeader:header_filter(conf)
    -- do custom logic here
    kong.response.set_header("myheader1", conf.header_value)
end

return MyHeader
