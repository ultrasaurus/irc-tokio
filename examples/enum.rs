use tokio::io::{AsyncRead, AsyncWriteExt};

#[derive(Debug)]
enum Value {
  Number(f64),
  Bool(bool)
}

impl Value {
  fn serialize<W: AsyncWriteExt>(self, bytes: W)
   {
    match self {
      Value::Bool(b) => {
        let val: u8;
        if b {
          val = 1;          
        } else {
          val = 0;
        }
        bytes.write_all(val.to_le_bytes());
      }
      val => bytes.write_all(val.to_ne_bytes()),
    }
  }
}


fn main() {

  if cfg!(target_endian = "little" ) {
    println!("little endian")
  } 
  if cfg!(target_endian = "big" ) {
    println!("big endian")
  }
  
  let x = Value::Number(4.0);

  println!("x = {:?}", x);

  let mut bytes = [0; 10];
  x.serialize(&mut bytes);
  println!("bytes = {:?}", bytes);

}