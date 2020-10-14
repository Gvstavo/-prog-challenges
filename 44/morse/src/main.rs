use std::fs::File;
use std::io::BufReader;
use rodio::Source;


fn main() {

	let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

 // let file = File::open("E_morse_code.ogg").unwrap();
 // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
 // stream_handle.play_raw(source.convert_samples());

  let file = File::open("T_morse_code.ogg").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
  stream_handle.play_raw(source.convert_samples());


  loop{}

}
