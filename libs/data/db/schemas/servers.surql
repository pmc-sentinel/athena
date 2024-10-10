DEFINE TABLE OVERWRITE modpack SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE name ON modpack TYPE string;
DEFINE FIELD OVERWRITE description ON modpack TYPE string;
DEFINE FIELD OVERWRITE source ON modpack TYPE string;

DEFINE TABLE OVERWRITE server SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE name ON server TYPE string;
DEFINE FIELD OVERWRITE port ON server TYPE int;
DEFINE FIELD OVERWRITE limit_fps ON server TYPE int;
DEFINE FIELD OVERWRITE extra_flags ON server TYPE array<string>;
DEFINE FIELD OVERWRITE world ON server TYPE string;
DEFINE FIELD OVERWRITE modpack ON server TYPE option<record<modpack>>;

DEFINE INDEX OVERWRITE port_uq ON TABLE server COLUMNS port UNIQUE;
