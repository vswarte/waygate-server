struct RequestGetItemLogParamsEntry {
    MapRelativeLocation location;
    ItemCategory itemCategory;
    u32 itemId;
    u32 quantityAcquired;
    u32 unk1;
    u32 unk2;
    u32 unk3;
};

using RequestGetItemLogParams = Vector<RequestGetItemLogParamsEntry>;

using ResponseGetItemLogParams = ResponseOk;
