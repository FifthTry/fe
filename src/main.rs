fn main() {
    let content = fte::editor::editor("", None).unwrap();
    dbg!(&content);
}
