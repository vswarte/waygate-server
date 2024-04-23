struct RequestGetGhostDataListParams {
    Vector<OnlineArea> searchAreas;
    GroupPasswords groupPasswords;
};

struct ResponseGetGhostDataListParamsEntry {
    OnlineArea area;
    ObjectIdentifier identifier;
    Vector<char> data;
    GroupPasswords groupPasswords;
};

struct ResponseGetGhostDataListParams {
    Vector<ResponseGetGhostDataListParamsEntry> entries;
};
