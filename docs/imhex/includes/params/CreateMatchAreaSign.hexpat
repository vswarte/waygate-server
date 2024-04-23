struct RequestCreateMatchAreaSignParams {
    u32 unk1;
    u32 unk2;
    u32 unk3;
    MatchingParameters MatchingParameters;
    //OnlineArea area;
    //u32 unk0;
    //Vector<char> data;
    //GroupPasswords groupPasswords;
};

struct ResponseCreateMatchAreaSignParams {
    ObjectIdentifier identifier;
};
