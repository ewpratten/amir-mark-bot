//! I'm really too lazy to make this runtime-configurable just for the joke, so here is the dreaded constants file.

/// Users who may edit amir's grades
pub const AUTHORIZED_USERS: [u64; 2] = [
    // Me
    375371353085444097,
    // Dario
    749115534045610095,
];

/// Amir's UID
pub const AMIR_ID: u64 = 263789571425304576;

/// Class GID
pub const GUILD_ID: u64 = 1062491599713607840; // Prod guild
// pub const GUILD_ID: u64 = 1070710208814391337; // Test guild

/// Path to the file containing Amir's current grade
pub const GRADE_FILE_PATH: &str = "/grades/amir_grade.txt"; // Prod path
// pub const GRADE_FILE_PATH: &str = "./amir_grade.txt"; // Test path