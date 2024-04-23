struct RequestKillEnemyLogParamsEntry {
    u32 entityId;
    u32 unk4;
};

struct RequestKillEnemyLogParams {
    Vector<RequestKillEnemyLogParamsEntry> killedEnemies;
    MapRelativeLocation location;
};

using ResponseKillEnemyLogParams = ResponseOk;
