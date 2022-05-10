use super::parse;

#[test]
fn test_parse() {
    let content = "#EXTM3U
#EXT-X-TARGETDURATION:10
#EXT-X-VERSION:3
#EXTINF:9.009,
first.ts
#EXTINF:9.009,
second.ts
#EXTINF:3.003,
third.ts
#EXT-X-ENDLIST";
    let url = "http://media.example.com/fds/fds/fds/654654/-0fds.fdg4.432/indx.m3u8?fuck";
    let ret = parse(url, content);
    panic!("{:?}", ret);
}
