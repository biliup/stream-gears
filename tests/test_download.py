import stream_gears


class Segment:
    pass


if __name__ == '__main__':
    segment = Segment()
    # segment.time = 60
    segment.size = 6000 * 1024 * 1024
    stream_gears.download(
        "url",
        # {"referer": "https://live.bilibili.com"},
        {},
        "new_test",
        segment
    )
