struct RequestGetBloodstainListParams {
    Vector<OnlineArea> searchAreas;
    GroupPasswords groupPasswords;
};

struct ResponseGetBloodstainListParamsEntry {
    OnlineArea area;
    ObjectIdentifier identifier;
    Vector<char> data;
    GroupPasswords groupPasswords;
};

struct ResponseGetBloodstainListParams {
    Vector<ResponseGetBloodstainListParamsEntry> entries;
};
