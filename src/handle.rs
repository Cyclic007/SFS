use std::{collections::HashMap, sync::{Arc, Mutex},fs::File,ffi::OsString,ffi::OsStr};
use log::{debug, error};
use std::ptr;
use std::path::PathBuf;
use lazy_static::lazy_static;
use super::driveActions::{start_block_read,data_block_read,get_data_block_from_start_block};
use super::blocks::{StartBlock,DataBlock,RawDataBlock};
#[derive(Clone)]
pub struct FileHandle {
    pub path: Box<std::path::Path>,
    pub start_block_index: u32 
}




// I love Doors
pub struct HandleStorage {
	current : HashMap<u64, FileHandle>,
	nextNum : u64,
	free : Vec<u64>
	//FREDOM FROM OPRESSION
}


impl HandleStorage {

	fn new() -> Self {
		HandleStorage {
			current : HashMap::new(),
			nextNum : 0,
			free :  Vec::new(),
		}
	}

	fn new_handle(&mut self, item : FileHandle) -> u64{
		let num = self.get_next_free();
		assert!(self.current.insert(num,item).is_none(),"error: handle location already alocated");
		num
	}

	fn get_next_free(&mut self) -> u64{
		if self.free.is_empty(){
			let num = self.nextNum;
			self.nextNum += 1;
			return num;
		}

		self.free.pop().expect("magic")
	}

	fn remove_handle(&mut self, num : u64){
		if self.current.remove(&num).is_none() {
			error!("removed something that does not exist");
		}
		if num == self.nextNum{
			self.nextNum -= 1;
		}
	}

	fn read_handle_block_index(&self, num : u64) -> u32{
		if let Some(_hand) = self.current.get(&num){
			self.current.get(&num).unwrap().start_block_index
		}else {
			error!("handle does not exist");
			panic!("handle does not exist")
		}
		
	}

	fn read_handle(&self , num:u64) -> FileHandle{
		if let Some(_hand) = self.current.get(&num){
			self.current.get(&num).unwrap().clone()
		}else {
			error!("handle does not exist");
			panic!("handle does not exist ")
		}
	}



}




lazy_static! {
    static ref CURRENT_HANDLES: Arc<Mutex<HandleStorage>> = Arc::new(Mutex::new(HandleStorage::new()));
}



impl FileHandle {

	pub fn new(path: Box<std::path::Path>) -> Self{
		FileHandle{path,start_block_index: 0}
	}

    pub fn name(&self) -> &str {
        // Get the name, if it exists.
        if let Some(name) = self.path.file_name() {
            name.to_str().expect("Should be valid UTF8")
        } else {
            // No name, this must be the root.
            ""
        }
    }


	//this will add the handle to storage
	//will lock
	pub fn allocate(self) -> u64{
		let store = &mut CURRENT_HANDLES.lock().expect("Other mutex holders should not panic.");
		store.new_handle(self)
	}

	pub fn allocate_with_index(mut self, file: File) -> u64{
		self.start_block_index = self.clone().get_start_block_index(&file);
		self.allocate()
	}

    
	pub fn read(number : u64) -> Self{
		let store = CURRENT_HANDLES.lock().expect("Other mutex holders should not panic.");
		store.read_handle(number)
	} 
	pub fn read_handle_index(number : u64) -> u32{
		let store = CURRENT_HANDLES.lock().expect("Other mutex holders should not panic.");
		store.read_handle_block_index(number)
	}


	pub fn drop_handle(number : u64){
		let store = &mut CURRENT_HANDLES.lock().expect("Other mutex holders should not panic.");
		store.remove_handle(number);
	}


	// this returns (data_block_index , directory entry slot)
	pub fn get_directory_entry(&self, file : &File) -> (u32 , u32){
		let parent_path = (*self.path).parent().expect("this path has no parents");
		let mut current_start_block = start_block_read(file,0);
		let mut path_vec : Vec<OsString> = Vec::with_capacity(10);
		let mut before_start_block = start_block_read(file,0);
		//this means that we now have a vector with all of the parts of the path
		'outside : for part in parent_path.to_path_buf().iter(){
			if part.to_str().expect("this part is not a string") == "/"{
				continue 'outside;
			}

			
			let current_data_block = get_data_block_from_start_block(file,&current_start_block);
			let directory_ptrs = current_data_block.parse_to_directory_ptrs(&file);
			for ptr in directory_ptrs{
				let tmp_start_block = start_block_read(file,ptr);
				println!("{},{}",tmp_start_block.get_name().to_str().expect("this name is not a string"),part.to_str().expect("this part is not a string"));
				if tmp_start_block.get_name().to_str().expect("this name is not a string") == part.to_str().expect("this part is not a string"){
					println!("found the name of the current part");
					before_start_block = current_start_block;
					current_start_block = tmp_start_block;
					
					continue 'outside;
				}
			}
			println!("this file does not exist from handles");
		}
		(before_start_block.blockPosition,data_block_read(file,before_start_block.firstDataBlockPos).find_directory_entry_slot(current_start_block.blockPosition).expect("this block does not exist"))	
	}



	
	pub fn get_start_block_index(&mut self, file : &File) -> u32{

		let _temp_str = (*self.path).to_str().expect("this path is not a string");
		println!("{}",(*self.path).to_str().expect("this path is not a string"));
		if (*self.path).to_str().expect("this path is not a string") == "/"{
			println!("this is the root dir");
			self.start_block_index = 0;
			return 0;
		}else{
			// first you start with the root block
			let mut current_start_block = start_block_read(file,0);
			let mut path_vec : Vec<OsString> = Vec::with_capacity(10);
			
			//this means that we now have a vector with all of the parts of the path
			'outside : for part in self.path.to_path_buf().iter(){
				if part.to_str().expect("this part is not a string") == "/"{
					continue 'outside;
				}

				
				let current_data_block = get_data_block_from_start_block(file,&current_start_block);
				let directory_ptrs = current_data_block.parse_to_directory_ptrs(&file);
				for ptr in directory_ptrs{
					let tmp_start_block = start_block_read(file,ptr);
					println!("{},{}",tmp_start_block.get_name().to_str().expect("this name is not a string"),part.to_str().expect("this part is not a string"));
					if tmp_start_block.get_name().to_str().expect("this name is not a string") == part.to_str().expect("this part is not a string"){
						println!("found the name of the current part");
						current_start_block = tmp_start_block;
						continue 'outside;
					}
				}
				println!("this file does not exist from handles");
				return u32::MAX
			}
			self.start_block_index = current_start_block.blockPosition;
			return current_start_block.blockPosition;
			

			
			
		}
		
	}



}
