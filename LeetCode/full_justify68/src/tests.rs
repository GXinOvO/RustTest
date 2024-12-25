use super::full_justify;

#[test]
fn test_full_justify() {
    let words = vec![
        String::from("This"),
        String::from("is"),
        String::from("an"),
        String::from("example"),
        String::from("of"),
        String::from("text"),
        String::from("justification."),
    ];
    let max_width = 16;

    let result = vec![
        String::from("This    is    an"),
        String::from("example  of text"),
        String::from("justification.  "),
    ];

    assert_eq!(full_justify(words, max_width), result)

}