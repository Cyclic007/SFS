use std::{collections::HashMap, sync::{Arc, Mutex},fs::File,ffi::OsString,ffi::OsStr};
use log::{debug, error};
use std::ptr;

use lazy_static::lazy_static;
use super::driveActions::{start_block_read,data_block_read};
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
			panic!("handle does not exist ")
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
	
	pub fn get_start_block_index(mut self, file : File) -> u32{
		let mut start_block_indexes_to_check : Vec<u32> = vec!(1);
		start_block_indexes_to_check.clear();
		let mut data_block_indexes_to_check : Vec<u32> = vec!(1);
		data_block_indexes_to_check.clear();
		//init the root block
		let mut current_start_block = start_block_read(&file,0);
		let data_store = &file.try_clone();


		data_block_indexes_to_check.push(current_start_block.get_data_start_pos());
		let mut current_data_block = data_block_read(&data_store.as_ref().unwrap(),data_block_indexes_to_check.pop().unwrap());
		let mut current_directory_ptrs : Vec<u32> = current_data_block.parse_to_directory_ptrs(&file);
		let mut path_vec : Vec<OsString> = vec!(OsString::from("0"));
		path_vec.clear();
		loop {
			let mut temp_path = self.path.as_ref();
			path_vec.push(OsString::from(temp_path.file_name().unwrap()));
			temp_path = &temp_path.parent().unwrap();
			if temp_path.as_os_str().is_empty(){
				break
			}
		}




		'outer: for part in path_vec{
			start_block_indexes_to_check.append(&mut current_directory_ptrs);		
			current_directory_ptrs.clear();
			for _i in 0..start_block_indexes_to_check.len(){
				current_start_block = start_block_read(&data_store.as_ref().unwrap(),start_block_indexes_to_check.pop().unwrap());
				if current_start_block.get_name() == part.as_os_str().to_str().unwrap().to_string(){
						current_data_block = data_block_read(&data_store.as_ref().unwrap(),current_start_block.get_data_start_pos());
						current_directory_ptrs = current_data_block.parse_to_directory_ptrs(&data_store.as_ref().unwrap());
						start_block_indexes_to_check.clear();
						continue 'outer;
				}
			}
			return 0
			
		}
		self.start_block_index = current_start_block.blockPosition;
		return current_start_block.blockPosition;
		
	}



}
