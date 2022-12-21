enum Message {
    Quit, // has no data associated with it at all.
    Move { x: i32, y: i32 }, // has named fields like a struct does.
    Write(String), // includes a single String.
    ChangeColor(i32, i32, i32), // includes three i32 values.
}

// defining different kinds of struct
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
