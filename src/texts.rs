pub(crate) fn derpibooru_deserealized() -> String {
r##"Object(
    {
        "image": Object(
            {
                "animated": Bool(
                    false,
                ),
                "aspect_ratio": Number(
                    1.3751962323390892,
                ),
                "comment_count": Number(
                    2.0,
                ),
                "created_at": String(
                    "2012-01-20T02:54:19Z",
                ),
                "deletion_reason": Null,
                "description": String(
                    "",
                ),
                "downvotes": Number(
                    1.0,
                ),
                "duplicate_of": Null,
                "duration": Number(
                    0.04000000000000001,
                ),
                "faves": Number(
                    21.0,
                ),
                "first_seen_at": String(
                    "2012-01-20T02:54:19Z",
                ),
                "format": String(
                    "jpg",
                ),
                "height": Number(
                    2548.0,
                ),
                "hidden_from_users": Bool(
                    false,
                ),
                "id": Number(
                    1024.0,
                ),
                "intensities": Object(
                    {
                        "ne": Number(
                            181.037809,
                        ),
                        "nw": Number(
                            178.61847600000002,
                        ),
                        "se": Number(
                            174.34748000000002,
                        ),
                        "sw": Number(
                            228.098875,
                        ),
                    },
                ),
                "mime_type": String(
                    "image/jpeg",
                ),
                "name": String(
                    "1024__safe_rarity_artist-colon-rabidpeach",
                ),
                "orig_sha512_hash": Null,
                "processed": Bool(
                    true,
                ),
                "representations": Object(
                    {
                        "full": String(
                            "https://derpicdn.net/img/view/2012/1/20/1024.jpg",
                        ),
                        "large": String(
                            "https://derpicdn.net/img/2012/1/20/1024/large.jpg",
                        ),
                        "medium": String(
                            "https://derpicdn.net/img/2012/1/20/1024/medium.jpg",
                        ),
                        "small": String(
                            "https://derpicdn.net/img/2012/1/20/1024/small.jpg",
                        ),
                        "tall": String(
                            "https://derpicdn.net/img/2012/1/20/1024/tall.jpg",
                        ),
                        "thumb": String(
                            "https://derpicdn.net/img/2012/1/20/1024/thumb.jpg",
                        ),
                        "thumb_small": String(
                            "https://derpicdn.net/img/2012/1/20/1024/thumb_small.jpg",
                        ),
                        "thumb_tiny": String(
                            "https://derpicdn.net/img/2012/1/20/1024/thumb_tiny.jpg",
                        ),
                    },
                ),
                "score": Number(
                    30.0,
                ),
                "sha512_hash": String(
                    "d583d0b4a27625052eeee0ef6baab365e2bdce40965afc076df9d41d82db4559253e709f8d738fe6e4e97269c12aedc8b3074a149a26e95c1afd14d9dcfe804a",
                ),
                "size": Number(
                    1107249.0,
                ),
                "source_url": String(
                    "http://rabidpeach.deviantart.com/art/Haircut-275691171",
                ),
                "source_urls": Array(
                    [
                        String(
                            "http://rabidpeach.deviantart.com/art/Haircut-275691171",
                        ),
                    ],
                ),
                "spoilered": Bool(
                    false,
                ),
                "tag_count": Number(
                    13.0,
                ),
                "tag_ids": Array(
                    [
                        Number(
                            13327.0,
                        ),
                        Number(
                            27141.0,
                        ),
                        Number(
                            30060.0,
                        ),
                        Number(
                            33983.0,
                        ),
                        Number(
                            38185.0,
                        ),
                        Number(
                            38764.0,
                        ),
                        Number(
                            39318.0,
                        ),
                        Number(
                            39435.0,
                        ),
                        Number(
                            40482.0,
                        ),
                        Number(
                            42350.0,
                        ),
                        Number(
                            46439.0,
                        ),
                        Number(
                            182100.0,
                        ),
                        Number(
                            261205.0,
                        ),
                    ],
                ),
                "tags": Array(
                    [
                        String(
                            "artist:rabidpeach",
                        ),
                        String(
                            "female",
                        ),
                        String(
                            "high res",
                        ),
                        String(
                            "mare",
                        ),
                        String(
                            "pony",
                        ),
                        String(
                            "profile",
                        ),
                        String(
                            "raised hoof",
                        ),
                        String(
                            "rarity",
                        ),
                        String(
                            "safe",
                        ),
                        String(
                            "solo",
                        ),
                        String(
                            "unicorn",
                        ),
                        String(
                            "smiling",
                        ),
                        String(
                            "photoshop elements",
                        ),
                    ],
                ),
                "thumbnails_generated": Bool(
                    true,
                ),
                "updated_at": String(
                    "2019-07-15T15:58:42Z",
                ),
                "uploader": Null,
                "uploader_id": Null,
                "upvotes": Number(
                    31.0,
                ),
                "view_url": String(
                    "https://derpicdn.net/img/view/2012/1/20/1024__safe_artist-colon-rabidpeach_rarity_pony_unicorn_female_high+res_mare_photoshop+elements_profile_raised+hoof_smiling_solo.jpg",
                ),
                "width": Number(
                    3504.0,
                ),
                "wilson_score": Number(
                    0.7801796140720005,
                ),
            },
        ),
        "interactions": Array(
            [],
        ),
    },
)"##.to_owned()
}

pub(crate) fn menu_string() -> String {
r##"{"menu": {
"id": "file",
"value": "File",
"popup": {
    "menuitem": [
    {"value": "New", "onclick": "CreateDoc()"},
    {"value": "Open", "onclick": "OpenDoc()"},
    {"value": "Save", "onclick": "SaveDoc()"}
    ]
}
}}
"##.to_owned()
}

pub(crate) fn menu_deserealized() -> String {
r##"Object(
    {
        "menu": Object(
            {
                "id": String(
                    "file",
                ),
                "popup": Object(
                    {
                        "menuitem": Array(
                            [
                                Object(
                                    {
                                        "onclick": String(
                                            "CreateDoc()",
                                        ),
                                        "value": String(
                                            "New",
                                        ),
                                    },
                                ),
                                Object(
                                    {
                                        "onclick": String(
                                            "OpenDoc()",
                                        ),
                                        "value": String(
                                            "Open",
                                        ),
                                    },
                                ),
                                Object(
                                    {
                                        "onclick": String(
                                            "SaveDoc()",
                                        ),
                                        "value": String(
                                            "Save",
                                        ),
                                    },
                                ),
                            ],
                        ),
                    },
                ),
                "value": String(
                    "File",
                ),
            },
        ),
    },
)"##.to_owned()
}
