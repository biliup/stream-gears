import stream_gears

a=[{
    'type':1,
    'raw_text': "wdadadadad",
    'biz_id': ""
},{
    'type':2,
    'raw_text': "健忘症10086",
    'biz_id': "4141"
},{
    'type':1,
    'raw_text': " 1321wada",
    'biz_id': ""
}]
if __name__ == '__main__':
    stream_gears.upload(
        ["test.flv"],
        "cookies.json",
        "title",
        171,
        "tag",
        1,
        "source",
        "wdadadadad@健忘症10086  1321wada",
        "dynamic",
        "",
        None,
        stream_gears.UploadLine.Bda2,
        3,
        a
    )
