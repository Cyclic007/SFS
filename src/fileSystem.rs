use std::ffi::{CStr, CString, OsStr, OsString};
use fuse_mt::*;
use log::Log;
pub struct SecureFileSystem{
	pub target: OsString,
}
use std::time::Duration;
use std::io::prelude::*;
use super::handle::FileHandle;
use std::io;
use std::path::Path;
use std::fs::File;
use std::time::SystemTime;
//use super::handle;
use super::driveActions::direct_block_read;
use super::driveActions::{direct_block_write,start_block_read,data_block_read};
use super::blocks::{RawDataBlock,RawBlock,StartBlock,DataBlock,MetaData};
impl SecureFileSystem{
	pub fn new( os : OsString) -> Self{
		Self{
			target : os,
		}
	}
	
}


impl FilesystemMT for SecureFileSystem{
	fn init(&self, req: RequestInfo) -> ResultEmpty {
		// first we need to load the root directory block into memeory
		//let mut main = File::open("").expect("SHHH");
		
		//let _rootBlock = direct_block_read( &File::open("/").expect("SHHH"),0);
		
		let mut root = File::options()
				    .read(true)
				    .write(true)
				    .open(self.target.clone()).unwrap();
				    		
		let root_block = direct_block_read(&root,0);
		let root_start_block = StartBlock::from(RawDataBlock::from(root_block));
		if root_start_block.hash == [0; 32]{
			println!("the file system is new");
			direct_block_write(&root,RawDataBlock::from(RawBlock::from(StartBlock::new([0;32],0,42,[0;247],MetaData::new(512,1,0,0,0,9,req.uid,req.gid,0),1,[0;32],2,[0;32],[0;81]))),0);
			println!("made new root block");
		} 


		println!("init");
		Ok(())
	}


	fn getattr(
        &self,
        _req: fuse_mt::RequestInfo,
        path: &std::path::Path,
        fh: Option<u64>,
    ) -> fuse_mt::ResultEntry {
	    let mut root = File::options()
	    		    .read(true)
	    		    .write(true)
	    		    .open(self.target.clone()).unwrap();
		let now = SystemTime::now();
		let handle = FileHandle::new(Box::from(path));
		if let Some(handleIndex) = fh{
			println!("handle was provided");
			Ok((
				now.elapsed().unwrap(),
				FileAttr::from(start_block_read(&root,FileHandle::read_handle_index(handleIndex)).get_attributes())
			))
		}else {
			
		}

		
						
	}
	
}	

