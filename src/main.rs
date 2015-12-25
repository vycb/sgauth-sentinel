use std::process::Command;
use std::time::Duration;
use std::thread::sleep;


fn main() {
	let done = false;
	let mut sgauth = Command::new("/home/Progs/sgauth")
		.arg( "/home/Progs/sgauth.conf")
		.spawn()
		.unwrap_or_else(|e| { panic!("faioled to execute sgauth: {}", e) });
		
	while !done {
		 sleep(Duration::from_millis(30000));
		 
		let mut chping = Command::new("ping")
		.arg( "-c 1")
		.arg( "google.com")
		.spawn()
		.unwrap_or_else(|e| { panic!("failed to execute ping: {}", e) });
		
		let ecode = chping.wait()
		.unwrap_or_else(|e| { panic!("failed to wait on ping: {}", e) });
		
		if !ecode.success() {
			
			match sgauth.kill(){
				Ok(_) => {
					sgauth.wait().unwrap_or_else(|e| { panic!("failed to wait on sgauth exit: {}", e) });
					
					sgauth = Command::new("/home/Progs/sgauth")
					.arg( "/home/Progs/sgauth.conf")
					.spawn()
					.unwrap_or_else(|e| { sgauth.kill().unwrap_or_else(|e|{ panic!("faioled to kill sgauth: {}", e) });
					 panic!("faioled to start sgauth: {}", e) });
				},
				Err(r) => panic!("sgauth kill problem: {}", r)
			}
		}
	}
}
