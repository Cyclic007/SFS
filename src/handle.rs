use std::{collections::HashMap, sync::{Arc, Mutex}};
use log::{debug, error};
use std::ptr;
use lazy_static::lazy_static;

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




lazy_static! {
    static ref CURRENT_HANDLES: Arc<Mutex<HandleStorage>> = Arc::new(Mutex::new(HandleStorage::new()));
}



impl FileHandle {
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
	pub fn allocate(self) -> u128{
		let store = &mut CURRENT_HANDLES.lock().expect("Other mutex holders should not panic.");
		store.new_handle(self)
	}

    
	pub fn read(number : u128) -> Self{
		let store = &mut CURRENT_HANDLES.lock().expect("Other mutex holders should not panic.");
		store.read_handle(number)
	} 



	pub fn drop_handle(number : u128){
		let store = &mut CURRENT_HANDLES.lock().expect("Other mutex holders should not panic.");
		store.remove_handle(number);
	}

	pub fn is_file(&self) -> Result<Option<bool>, c_int>{
		let name: String = self.name().to_string();
		//TODO 
		Ok(Some(true)
	}
}
