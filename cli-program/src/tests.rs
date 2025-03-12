#[cfg(test)]
#[test]
fn test_file_has_contents() {
    use crate::arguments::Command;
    let args = Command {
        filepath: String::from("text.txt"),
        search: String::from("text"),
        append: String::from("lolz")
    };
    
    let contents = args.read_file();
    assert_ne!(contents, String::from(""), "Text file is empty.");
}