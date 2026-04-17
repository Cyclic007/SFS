use fuse_mt::*;
pub struct SecureFileSystem{}
use std::io;
use std::path::Path;
use std::fs::File;
//use super::handle;
use super::driveActions::direct_block_read;
use super::driveActions::direct_block_write;


impl FilesystemMT for SecureFileSystem{
	fn init(&self, _req: RequestInfo) -> ResultEmpty {
		// first we need to load the root directory block into memeory
		let _rootBlock = direct_block_read( &File::open("/").expect("SHHH"),0);
		//direct_block_write()

		Ok(())
	}


// 	fn getattr(
//         &self,
//         _req: fuse_mt::RequestInfo,
//         path: &std::path::Path,
//         fh: Option<u64>,
//     ) -> fuse_mt::ResultEntry {
// 
// 	
// 	}
	
}	

