struct RequestGetAnnounceMessageListParams {
    u32 maxEntries;
    u32 unk1;
    u32 unk2;
};

struct Message {
    u32 index;
    u32 order;
    u32 unk2;
    String title;
    String body;
    u64 publishedAt;
};

struct ResponseGetAnnounceMessageListParams {
    Vector<Message> list1;
    Vector<Message> list2;
};
