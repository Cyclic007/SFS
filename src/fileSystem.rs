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
use std::collections::VecDeque;
//use super::handle;
use super::driveActions::direct_block_read;
use super::driveActions::{direct_block_write,start_block_read,data_block_read,data_write,data_read,create_dir,create_file,delete_file};
use super::blocks::{RawDataBlock,RawBlock,StartBlock,DataBlock,MetaData};

impl SecureFileSystem{
	pub fn new( os : OsString) -> Self{
		Self{
			target : os,
		}
	}
	
}
// What I currently have implemented

// chown
// chmod
// truncate
// init
// access (as a shell needs full impl)
// open
// opendir 
// release
// releasedir
// readdir
// read (in almost a 1 liner)
// write
// mkdir

// I need to implement a total of 33 methods 

impl FilesystemMT for SecureFileSystem{
	fn init(&self, req: RequestInfo) -> ResultEmpty {
		// first we need to load the root directory block into memeory
		//let mut main = File::open("").expect("SHHH");
		
		//let _rootBlock = direct_block_read( &File::open("/").expect("SHHH"),0);
		
		let root = File::options()
				    .read(true)
				    .write(true)
				    .open(self.target.clone()).unwrap();
				    		
		let root_block = direct_block_read(&root,0);
		let root_start_block = StartBlock::from(root_block);
		if root_start_block.hash == [0; 32]{
			println!("the file system is new");
			create_dir(&root, OsString::from("/"), 551, 0 ,req.uid,req.gid);
//			direct_block_write(&root,RawDataBlock::from(RawBlock::from(StartBlock::new([0;32],0,42,[0;247],MetaData::new(512,1,0,0,0,9,req.uid,req.gid,0),1,[0;32],2,[0;32],[0;81]))),0);
			println!("made new root block");
		} 


		println!("init");
		Ok(())
	}


	fn getattr(
        &self,
        req: fuse_mt::RequestInfo,
        path: &std::path::Path,
        fh: Option<u64>,
    ) -> fuse_mt::ResultEntry {

		if self.access(req,path,4).is_err(){
			Err(1)
		}else {
		    let root = File::options()
		    		    .read(true)
		    		    .write(true)
		    		    .open(self.target.clone()).unwrap();
			let now = SystemTime::now();
			if let Some(handle_index) = fh{
				// yay a handle already exists and I can just use that
				println!("handle was provided");
				Ok((
					now.elapsed().unwrap(),
					FileAttr::from(start_block_read(&root,FileHandle::read_handle_index(handle_index)).get_attributes())
				))
			}else {
				// I need to make a new handle to get the stuff from 			
				let mut handle = FileHandle::new(Box::from(path));
				let block_pos = handle.get_start_block_index(&root.try_clone().unwrap());
				if block_pos == u32::MAX{
					Err(2)
				}else{
				
					Ok((
						now.elapsed().unwrap(),
						FileAttr::from(start_block_read(&root.try_clone().unwrap(), block_pos).get_attributes()),
					))
				}
			}
		}
	}
	// change permissions
	fn chmod(
        &self,
        req: RequestInfo,
        path: &Path,
        fh: Option<u64>,
        mode: u32,
    ) -> ResultEmpty { 
		if req.uid != 0 {
			println!("not root user not allowed to change permissions");
			Err(3)
		} else if mode > 4095{
			println!("this permission can't exist");
			Err(3)


		}else {
			
			//this means that root is changeing permissions
			let root = File::options()
				    		    .read(true)
				    		    .write(true)
				    		    .open(self.target.clone()).unwrap();
			if let Some(handle_index) = fh{
				// Handle is provided YAY
				let mut current_block = start_block_read(&root.try_clone().unwrap(),FileHandle::read_handle_index(handle_index));
				current_block.attributes.perm = mode as u16;
				direct_block_write(&root,RawDataBlock::from(RawBlock::from(current_block)),FileHandle::read_handle_index(handle_index));
				Ok(())
			} else {
				let mut handle = FileHandle::new(Box::from(path));
				let block_pos = handle.get_start_block_index(&root.try_clone().unwrap());
				if block_pos == u32::MAX{
					println!("file does not exist");
					Err(2)
				} else {
					let mut current_block = start_block_read(&root.try_clone().unwrap(),block_pos);
					current_block.attributes.perm = mode as u16;
					direct_block_write(&root,RawDataBlock::from(RawBlock::from(current_block)),block_pos);
					Ok(())
				}
			}
		}
    }

	fn chown(
	    &self,
	    req: RequestInfo,
	    path: &Path,
	    fh: Option<u64>,
	    uid: Option<u32>,
	    gid: Option<u32>,
	) -> ResultEmpty {
		if req.uid != 0 {
			println!("not root user not allowed to change permissions");
			Err(1)
		}else {
			
			//this means that root is changeing permissions
			let root = File::options()
				    		    .read(true)
				    		    .write(true)
				    		    .open(self.target.clone()).unwrap();
			if let Some(handle_index) = fh{
				// Handle is provided YAY
				let mut current_block = start_block_read(&root.try_clone().unwrap(),FileHandle::read_handle_index(handle_index));

				if let Some(new_uid) = uid {
					// we have a new uid so we change it on the block
					current_block.attributes.uid = new_uid;
				}
				if let Some(new_gid) = gid {
					// we have a new uid so we change it on the block
					current_block.attributes.gid = new_gid;
				}
				


				direct_block_write(&root,RawDataBlock::from(RawBlock::from(current_block)),FileHandle::read_handle_index(handle_index));
				Ok(())
			} else {
				let mut handle = FileHandle::new(Box::from(path));
				let block_pos = handle.get_start_block_index(&root.try_clone().unwrap());
				if block_pos == u32::MAX{
					println!("file does not exist");
					Err(2)
				} else {
					let mut current_block = start_block_read(&root.try_clone().unwrap(),block_pos);
					if let Some(new_uid) = uid {
						// we have a new uid so we change it on the block
						current_block.attributes.uid = new_uid;
					}
					if let Some(new_gid) = gid {
						// we have a new uid so we change it on the block
						current_block.attributes.gid = new_gid;
					}
				

					direct_block_write(&root,RawDataBlock::from(RawBlock::from(current_block)),block_pos);
					Ok(())
				}
			}
		}
		
	}


	// my handler Q
	// This message will self destruct in 30 seconds
	fn open(&self, _req: RequestInfo, path: &Path, flags: u32) -> ResultOpen {
		let root = File::options()
   		    .read(true)
   		    .write(true)
   		    .open(self.target.clone()).unwrap();
		let mut handle = FileHandle::new(Box::from(path));
		if handle.clone().get_start_block_index(&root.try_clone().unwrap()) == u32::MAX {
			println!("file does not exist big sad ");
			Err(2)
		} else {
			Ok((
				handle.allocate_with_index(root.try_clone().unwrap()),
				flags
			))
		}
		
	}
	fn opendir(
        &self,
        _req: RequestInfo,
        path: &Path,
        flags: u32,
    ) -> ResultOpen {
    	let root = File::options()
   		    .read(true)
   		    .write(true)
   		    .open(self.target.clone()).unwrap();
		let mut handle = FileHandle::new(Box::from(path));		
		if handle.clone().get_start_block_index(&root.try_clone().unwrap()) == u32::MAX {
			println!("file does not exist big sad ");
			Err(2)
		} else {
			if start_block_read(&root,handle.clone().get_start_block_index(&root.try_clone().unwrap())).attributes.fileType != 0b00001000 {
				println!("this is not a directory");
				Err(20)
			}else {

				println!("directory is being opened");
				// you have opened the crypt and got cursed
				Ok((
					handle.allocate_with_index(root.try_clone().unwrap()),
					flags
				))
			}
		}
    }
	// Bonds name
	// james name
	// Jams Bonde is Haveinf a Stronk
	fn release(
        &self,
        req: RequestInfo,
        path: &Path,
        fh: u64,
        _flags: u32,
        lock_owner: u64,
        flush: bool,
    ) -> ResultEmpty {
		if flush {
			let _res  = self.flush(req,path,fh,lock_owner);
		}	
		FileHandle::drop_handle(fh);
		Ok(())
	}
    fn releasedir(
        &self,
        _req: RequestInfo,
        _path: &Path,
        fh: u64,
        _flags: u32,
    ) -> ResultEmpty {
    	FileHandle::drop_handle(fh);
    	Ok(())
    }




	

	fn truncate(
        &self,
        req: RequestInfo,
        path: &Path,
        fh: Option<u64>,
        size: u64,
    ) -> ResultEmpty { 
		if self.access(req,path,4).is_err(){
			Err(404)	
		}else {
			
			//this means that root is changeing permissions
			let root = File::options()
				    		    .read(true)
				    		    .write(true)
				    		    .open(self.target.clone()).unwrap();
			if let Some(handle_index) = fh{
				// Handle is provided YAY
				let mut current_block = start_block_read(&root.try_clone().unwrap(),FileHandle::read_handle_index(handle_index));
				current_block.attributes.size = size;
				direct_block_write(&root,RawDataBlock::from(RawBlock::from(current_block)),FileHandle::read_handle_index(handle_index));
				Ok(())
			} else {
				let mut handle = FileHandle::new(Box::from(path));
				let block_pos = handle.get_start_block_index(&root.try_clone().unwrap());
				if block_pos == u32::MAX{
					println!("file does not exist");
					Err(3)
				} else {
					let mut current_block = start_block_read(&root.try_clone().unwrap(),block_pos);
					current_block.attributes.size = size;
					direct_block_write(&root,RawDataBlock::from(RawBlock::from(current_block)),block_pos);
					Ok(())
				}
			}
		}	

    }


	//TODO finish implemtation	
	fn access(
		&self,
		req: RequestInfo, 
		path: &Path, 
		mask: u32
	) -> ResultEmpty {
		if req.uid == 0 {
			println!("this is root");
			Ok(())
		} else {
			let root = File::options()
    		    .read(true)
    		    .write(true)
    		    .open(self.target.clone()).unwrap();
    		let mut needed_mode = 0;
			let mut handle = FileHandle::new(Box::from(path));
			let block_pos = handle.get_start_block_index(&root.try_clone().unwrap());
			let start_block = start_block_read(&root,block_pos);
			if start_block.attributes.uid == req.uid{
				needed_mode = start_block.attributes.perm / 64;
			} else if start_block.attributes.gid == req.gid{
				needed_mode = start_block.attributes.perm%8 / 8;
			} else {
				needed_mode = start_block.attributes.perm % 8;
			}
			if block_pos == u32::MAX {
				println!("file does not exist");
				Err(3)
			} else {
				if mask & u32::from(needed_mode) != 0{
					println!("{}",mask);
					Ok(())					
				}else {
					Err(1)
				}

			}
			
						
			
		}

	

	}


	// they read me like a book
	// and they played you like the cheap kazoo you are

	// this needs to get all of the names of the items in the direcory storage
	fn readdir(&self, req: RequestInfo, path: &Path, fh: u64) -> ResultReaddir{
		if self.access(req,path,4).is_err(){
			Err(404)	
		}else{
			let root = File::options()
			    		    .read(true)
			    		    .write(true)
			    		    .open(self.target.clone()).unwrap();
			let mut directory_ptrs : Vec<u32> = vec!(0,0);
			directory_ptrs.clear();
			println!("directory is being read");
			let handle = FileHandle::read(fh);
			let start_block = start_block_read(&root.try_clone().unwrap(),handle.clone().get_start_block_index(&root.try_clone().unwrap()));
			Ok(data_block_read(&root,start_block.get_data_start_pos()).parse_directory_to_directory_entry_struct_vector(&root,handle.clone()))
			
			
		}
		
	}
	fn read(
        &self,
        req: RequestInfo,
        path: &Path,
        fh: u64,
        offset: u64,
        size: u32,
        callback: impl FnOnce(ResultSlice<'_>) -> CallbackResult,
    ) -> CallbackResult {
    	if self.access(req,path,4).is_err(){
			callback(Err(404))	
		}else{
			let root = File::options()
    		    .read(true)
    		    .write(true)
    		    .open(self.target.clone()).unwrap();
			let block_offset : u32= u32::try_from(offset/468).unwrap();
			let inner_block_offset : usize  = usize::try_from(offset%468).unwrap();
			let start_block = start_block_read(&root.try_clone().unwrap(), FileHandle::read_handle_index(fh));
			//you know it's bad when you need to break a function call into multible lines
			callback(
				Ok(
					&data_read(
						&root.try_clone().unwrap(),
						data_block_read(&root.try_clone().unwrap(),start_block.get_data_start_pos()).get_data_block_pos_from_block_offset(
							&root.try_clone().unwrap(),
							block_offset
						),
						usize::try_from(size).unwrap(),
						match size{
							0..448 => usize::try_from(size).unwrap(),
							_ => 448
						},
						inner_block_offset
								
					)[..]
				)
			)
		}	
    }



	
    

	// I am a Published Author
	// --Doug Doug
	fn write(
	    &self,
	    req: RequestInfo,
	    path: &Path,
	    fh: u64,
	    offset: u64,
	    data: Vec<u8>,
	    _flags: u32,
	) -> ResultWrite {
		if self.access(req,path,2).is_err(){
			Err(404)	
		}else{
			let root = File::options()
    		    .read(true)
    		    .write(true)
    		    .open(self.target.clone()).unwrap();
			
			Ok(
				data_write(
					&root,
					VecDeque::from(data.clone()),
					usize::try_from(offset).unwrap(),
					data.len(),
					FileHandle::read_handle_index(fh),
					5000,
					448,
					0
				)

			)
			
		} 		
	}


	fn mkdir(
	    &self,
	    req: RequestInfo,
	    parent: &Path,
	    name: &OsStr,
	    mode: u32,
	) -> ResultEntry {
		if self.access(req,parent,2).is_err(){
			Err(404)	
		}else{
			let now = SystemTime::now();
			println!("attempting to create directory at {}",parent.to_str().expect("unable to parse parent path to str"));    		    
						
			
			let mut tempHandle = FileHandle::new(Box::from(parent));
			let root = File::options()
    		    .read(true)
    		    .write(true)
    		    .open(self.target.clone()).expect("unable to open file");

			let mut check_handle = FileHandle::new(Box::from(parent.join(name)));
			if check_handle.get_start_block_index(&root) != u32::MAX{
				println!("the parent has no start block");
				Err(2)
			}else{
			
				Ok((now.elapsed().unwrap(),FileAttr::from(create_dir(&root, name.to_os_string(), mode, tempHandle.get_start_block_index(&root),req.uid,req.gid))))
			}
		}			
	}

	fn mknod(&self, req: RequestInfo, parent: &Path, name: &OsStr, mode: u32, rdev: u32) -> ResultEntry {
	    if self.access(req,parent,2).is_err(){
   			Err(404)	
   		}else{
   			let now = SystemTime::now();
   			println!("attempting to create file at {}",parent.to_str().expect("unable to parse parent path to str"));    		    
   			
   			let root = File::options()
       		    .read(true)
       		    .write(true)
       		    .open(self.target.clone()).expect("unable to open file");
      			
   			let mut temp_handle = FileHandle::new(Box::from(parent));
			let mut check_handle = FileHandle::new(Box::from(parent.join(name)));

			if check_handle.get_start_block_index(&root) != u32::MAX{
				Err(2)
			}else{
				if temp_handle.get_start_block_index(&root) == u32::MAX{
					Err(2)
				}else{
				
		   			Ok((now.elapsed().unwrap(),FileAttr::from(create_file(&root, name.to_os_string(), u16::try_from(mode).expect("mode is not 16 bit"), temp_handle.get_start_block_index(&root),req.uid,req.gid))))
				}
			}
   		}
	}



	fn unlink(&self, req: RequestInfo, parent: &Path, name: &OsStr) -> ResultEmpty {
	    if self.access(req,parent,2).is_err(){
   			Err(404)	
   		}else {
   			let root = File::options()
	   		    .read(true)
	   		    .write(true)
	   		    .open(self.target.clone()).expect("unable to open file");
   			let mut tmp_parent_handle = FileHandle::new(Box::from(parent));
			let tmp_parent_index = tmp_parent_handle.get_start_block_index(&root);
   			let mut tmp_to_be_del_handle = FileHandle::new(Box::from(parent.join(name)));
			let tmp_to_be_del_index = tmp_to_be_del_handle.get_start_block_index(&root);
			
   			delete_file(&root,tmp_to_be_del_index,tmp_parent_index);
   			Ok(())
   		}
	}

	fn rmdir(&self, req: RequestInfo, parent: &Path, name: &OsStr) -> ResultEmpty {
	    if self.access(req,parent,2).is_err(){
   			Err(404)	
   		}else {
   			let root = File::options()
	   		    .read(true)
	   		    .write(true)
	   		    .open(self.target.clone()).expect("unable to open file");
   			let mut tmp_parent_handle = FileHandle::new(Box::from(parent));
			let tmp_parent_index = tmp_parent_handle.get_start_block_index(&root);
   			let mut tmp_to_be_del_handle = FileHandle::new(Box::from(parent.join(name)));
			let tmp_to_be_del_index = tmp_to_be_del_handle.get_start_block_index(&root);
			
   			delete_file(&root,tmp_to_be_del_index,tmp_parent_index);
   			Ok(())
   		}

	}

//pub fn create_dir(driveFile : &File, name : OsString, mode : u32, parent_start_index : u32, uid : u32, gid : u32) -> MetaData{
	
}	

