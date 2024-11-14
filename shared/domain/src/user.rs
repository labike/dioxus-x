use nutype::nutype;

#[nutype(
    validate(not_empty, len_char_min = 3, len_char_max = 30),
    derive(AsRef, Clone, Debug, PartialEq, Serialize, Deserialize)
)]
// Asref允许访问字符串自身
pub struct Username(String);

#[nutype(
    validate(not_empty, len_char_min = 8),
    derive(AsRef, Clone, Debug, PartialEq, Serialize, Deserialize)
)]
pub struct Password(String);