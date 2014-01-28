use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
	let fname =  args[1].clone();
	let fname2 = args[2].clone();
        let path = Path::new(fname.clone());
	let path2 = Path::new(fname2.clone());
        let msg_file = File::open(&path);
	let msg2_file = File::open(&path2);

        match (msg_file, msg2_file) {
            (Some(mut msg), Some(mut msg2)) => {
                let msg_bytes: ~[u8] = msg.read_to_end();
		let msg2_bytes: ~[u8] = msg2.read_to_end();
                let decoded_file 
                       = File::create(&Path::new("decoded" + ".txt"));
                
                match (decoded_file) {
                    Some(decoded) => { 
                        join(msg_bytes, msg2_bytes, decoded); 
                        } ,
                    None => fail!("Error opening output file!"),
 }
            } ,
            (_,_) => fail!("Error opening message file: {:s}", fname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(msg_bytes: &[u8], msg2_bytes: &[u8], mut decoded: File) {
 
    let joined = xor(msg_bytes, msg2_bytes);
    decoded.write(joined);

}




