use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
        let mut vec = ~args;
        let fname1 = vec[1].clone();
        let fname2 = vec[2].clone();
        let path1 = Path::new(fname1.clone());
        let path2 = Path::new(fname2.clone());
        let share1 = File::open(&path1);
        let share2 = File::open(&path2);

        match (share1, share2) {
            (Some(mut msg1), Some(mut msg2)) => {
                let msg1_bytes: ~[u8] = msg1.read_to_end();
                let msg2_bytes: ~[u8] = msg2.read_to_end();
                let joined_file 
                       = File::create(&Path::new(fname1 + ".txt"));
                
                match (joined_file) {
                    Some(joined_file) => { 
                        join(msg1_bytes, msg2_bytes, joined_file); 
                        } ,
                    _ => fail!("Error opening output files!"),
                }
            } ,
            (_, _) => fail!("Error opening message files: {:s}, {:s}", fname1, fname2)
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

fn join(msg1_bytes: &[u8], msg2_bytes: &[u8], mut joined_file: File) {
    let decrypted_bytes = xor(msg1_bytes, msg2_bytes);
    joined_file.write(decrypted_bytes);
}