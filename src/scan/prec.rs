//! 运算符优先级

/// 返回符号优先级
pub fn prec(x:&[u8])-> u8 {
  match x {
    b"*" | b"%" | b"/" => 11, 
    b"+" | b"-" => 10, 
    b"<<"|b">>" => 9,
    b"&" => 8,
    b"^" => 7,
    b"|" => 6,
    b"=="|b"!="|b"<"|b">"|b"<="|b">=" => 5,
    b"&&" => 4,
    b"||" => 3,
    b"="|b"+="|b"-="|b"*="|b"/="|b"%="|b"&="|b"|="|b"^="|b"<<="|b">>=" => 2,
    b"," => 1, 
    _=> 0
  }
}
