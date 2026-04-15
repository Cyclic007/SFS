use std::collections::HashMap;

use log::{debug, error};
use std::ptr;


pub struct FileHandle {
    pub path: Box<std::path::Path>,
}

// I love Doors
pub struct HandleStorage {
	current : HashMap<u128, FileHandle>,
	nextNum : u128,
	free : Vec<u128>
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

	fn new_handle(&mut self, item : FileHandle) -> u128{
		let num = self.get_next_free();
		assert!(self.current.insert(num,item).is_none(),"error: handle location already alocated");
		num
	}

	fn get_next_free(&mut self) -> u128{
		if self.free.is_empty(){
			let num = self.nextNum;
			self.nextNum += 1;
			return num;
		}

		self.free.pop().expect("magic")
	}

	fn remove_handle(&mut self, num : u128){
		if self.current.remove(&num).is_none() {
			error!("removed something that does not exist");
		}
		if num == self.nextNum{
			self.nextNum -= 1;
		}
	}

	fn read_handle(&self, num : u128) -> FileHandle{
		if let Some(hand) = self.current.get(&num).clone(){
			unsafe {std::ptr::read(hand)}
		}else {
			error!("handle does not exist");
			panic!("handle does not exist ")
		}
		
	}



}
