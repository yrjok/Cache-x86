use std::fs::File;
use std::io::Write;
use std::io;

pub fn write_csv(values: &mut dyn Iterator<Item=u64>) -> io::Result<()> {
  let mut file = File::create("samples.csv")?;
  let mut samples = values.peekable();
  loop {
    if let Some(sample) = samples.next() {
      file.write(sample.to_string().as_bytes());
      if let Some(_) = samples.peek() {
        file.write(", ".as_bytes());
      }
    } else { break; }
  }
  file.flush();
  Ok(())
}