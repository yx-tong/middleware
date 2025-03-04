pub enum Rson {
    Void,
    Unit,
    Boolean {
        value: bool,
    },
    Integer8 {
        signed: bool,
        bytes: [u8; 1],
    },
    Integer16 {
        signed: bool,
        bytes: [u8; 2],
    },
    Integer32 {
        signed: bool,
        bytes: [u8; 4],
    },
    Integer64 {
        signed: bool,
        bytes: [u8; 8],
    },
    Integer128 {
        signed: bool,
        bytes: [u8; 16],
    },
    Integer256 {
        signed: bool,
        bytes: [u8; 32],
    },
    /// 'c'
    Unicode {
        text: char,
    },
    /// 'string'
    Text {
        text: String,
    },
    Interval {
        l: Box<Rson>,
        r: Box<Rson>,
    },
    List {
        items: Vec<Rson>,
    },
    Dict {
        items: IndexMap<String, Rson>,
    },
}
