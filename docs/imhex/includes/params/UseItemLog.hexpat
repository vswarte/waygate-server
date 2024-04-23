struct RequestUseItemLogParamsEntry {
    u32 goodsParamId;
    u32 quantity;
    u32 unk3;
};


struct RequestUseItemLogParams {
    Vector<RequestUseItemLogParamsEntry> itemsUsed;
    MapRelativeLocation location;
};

using ResponseUseItemLogParams = ResponseOk;
